import 'dart:convert';
import 'dart:io';

import 'package:hhm_pub_lib_core/hhm_pub_lib_core.dart';
import 'package:test/test.dart';

Map<String, dynamic> fixture(String name) {
  final text = File('../../conformance/cases/$name').readAsStringSync();
  return jsonDecode(text) as Map<String, dynamic>;
}

void main() {
  test('accepts shared valid cases', () {
    expect(
      validateJson('ClientInfo', fixture('valid-client-info.json')),
      isNotNull,
    );
    expect(
      validateJson('PublicLocation', fixture('valid-location.json')),
      isNotNull,
    );
    expect(
      validateJson('PublicAccountContext', fixture('valid-account.json')),
      isNotNull,
    );
  });

  test('rejects shared invalid cases', () {
    for (final entry in <(String, String)>[
      ('ClientInfo', 'invalid-extra-field.json'),
      ('ClientInfo', 'invalid-platform.json'),
      ('ClientInfo', 'invalid-oversized-version.json'),
      ('PublicAccountContext', 'invalid-uuid.json'),
      ('IdempotencyKey', 'invalid-int64.json'),
      ('ClientInfo', 'invalid-prototype-key.json'),
    ]) {
      expect(
        () => validateJson(entry.$1, fixture(entry.$2)),
        throwsA(isA<ContractValidationException>()),
      );
    }
  });

  test('unknown models fail closed', () {
    expect(
      () => validateJson('InternalGrant', <String, dynamic>{}),
      throwsA(
        isA<ContractValidationException>().having(
          (error) => error.code,
          'code',
          'unknown_model',
        ),
      ),
    );
  });

  final text = File('../../conformance/manifest.json').readAsStringSync();
  final manifest = jsonDecode(text) as Map<String, dynamic>;
  final cases = manifest['cases'] as List<dynamic>;
  test('fixture manifest has the expected version and is nonempty', () {
    expect(manifest['schemaVersion'], 'hhm.public-conformance.v1');
    expect(cases, isNotEmpty);
  });
  final names = <String>{};
  for (final raw in cases) {
    final entry = raw as Map<String, dynamic>;
    final file = entry['file'] as String;
    final model = entry['model'] as String;
    final valid = entry['valid'] as bool;
    if (!names.add(file) ||
        !file.endsWith('.json') ||
        file.contains('/') ||
        file.contains('\\')) {
      throw StateError('Invalid or duplicate fixture basename: $file');
    }
    test('shared manifest: $file', () {
      expect(
        model,
        isIn([
          'ClientInfo',
          'IdempotencyKey',
          'PublicLocation',
          'PublicAccountContext',
        ]),
      );
      if (valid) {
        expect(validateJson(model, fixture(file)), isNotNull);
      } else {
        expect(
          () => validateJson(model, fixture(file)),
          throwsA(isA<ContractValidationException>()),
        );
      }
    });
  }
}
