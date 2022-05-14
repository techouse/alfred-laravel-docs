class Config {
  Config._();

  static const String version = '2.0.7';
  static final Uri githubRepositoryUrl =
      Uri.https('github.com', '/techouse/alfred-laravel-docs');
  static const String algoliaApplicationId = 'E3MIRNPJH5';
  static const String algoliaSearchOnlyApiKey =
      '1fa3a8fec06eb1858d6ca137211225c0';
  static const String algoliaSearchIndex = 'laravel';
  static const Map<String, String> supportedVersions = {
    'v4.2': '4.2',
    'v5': '5.0',
    'v5.0': '5.0',
    'v5.1': '5.1',
    'v5.2': '5.2',
    'v5.3': '5.3',
    'v5.4': '5.4',
    'v5.5': '5.5',
    'v5.6': '5.6',
    'v5.7': '5.7',
    'v5.8': '5.8',
    'v6': '6',
    'v6.x': '6.x',
    'v7': '7.x',
    'v7.x': '7.x',
    'v8': '8.x',
    'v8.x': '8.x',
    'v9': '9.x',
    'v9.x': '9.x',
  };
}
