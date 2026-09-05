import 'src/generated/models.dart' as generated;

export 'src/generated/models.dart';

final class ContractValidationException implements Exception {
  const ContractValidationException(this.code);

  final String code;

  @override
  String toString() => 'ContractValidationException($code)';
}

Object validateJson(String model, Map<String, dynamic> json) {
  try {
    switch (model) {
      case 'ClientInfo':
        _exactKeys(json, const {
          'installId',
          'platform',
          'appVersion',
          'locale',
        });
        _required(json, const {'installId', 'platform', 'appVersion'});
        _bounded(json['installId'], 128);
        _bounded(json['appVersion'], 64);
        _optionalBounded(json['locale'], 16);
        return generated.ClientInfo.fromJson(json);
      case 'IdempotencyKey':
        _exactKeys(json, const {'key', 'mintedAtMs'});
        _required(json, const {'key', 'mintedAtMs'});
        _bounded(json['key'], 128);
        return generated.IdempotencyKey.fromJson(json);
      case 'PublicLocation':
        _exactKeys(json, const {
          'slug',
          'displayName',
          'city',
          'countryCode',
          'timezone',
          'availability',
        });
        _required(json, const {
          'slug',
          'displayName',
          'city',
          'countryCode',
          'timezone',
          'availability',
        });
        _bounded(json['slug'], 64);
        _bounded(json['displayName'], 120);
        _bounded(json['city'], 80);
        _bounded(json['countryCode'], 2);
        _bounded(json['timezone'], 64);
        return generated.PublicLocation.fromJson(json);
      case 'PublicAccountContext':
        _exactKeys(json, const {'id', 'kind', 'displayName'});
        _required(json, const {'id', 'kind', 'displayName'});
        _uuid(json['id']);
        _bounded(json['displayName'], 120);
        return generated.PublicAccountContext.fromJson(json);
      default:
        throw const ContractValidationException('unknown_model');
    }
  } on ContractValidationException {
    rethrow;
  } on Object {
    throw const ContractValidationException('invalid_shape');
  }
}

void _exactKeys(Map<String, dynamic> json, Set<String> allowed) {
  if (json.keys.any((key) => !allowed.contains(key))) {
    throw const ContractValidationException('unexpected_field');
  }
}

void _required(Map<String, dynamic> json, Set<String> required) {
  if (required.any((key) => !json.containsKey(key) || json[key] == null)) {
    throw const ContractValidationException('missing_field');
  }
}

void _bounded(Object? value, int maximum) {
  if (value is! String || value.length > maximum) {
    throw const ContractValidationException('constraint_violation');
  }
}

void _optionalBounded(Object? value, int maximum) {
  if (value != null) {
    _bounded(value, maximum);
  }
}

void _uuid(Object? value) {
  final pattern = RegExp(
    r'^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$',
  );
  if (value is! String || !pattern.hasMatch(value)) {
    throw const ContractValidationException('constraint_violation');
  }
}
