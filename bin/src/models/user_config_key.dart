import 'package:recase/recase.dart';

enum UserConfigKey {
  laravelVersion,
  useAlfredCache,
  useFileCache,
  cacheTtl,
  fileCacheMaxEntries;

  @override
  String toString() => name.snakeCase;
}
