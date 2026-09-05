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
}
