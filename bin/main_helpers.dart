part of 'main.dart';

final HtmlUnescape _unescape = HtmlUnescape();

final AlfredWorkflow _workflow = AlfredWorkflow();

final AlfredUpdater _updater = AlfredUpdater(
  githubRepositoryUrl: Uri.parse(Env.githubRepositoryUrl),
  currentVersion: Env.appVersion,
  updateInterval: Duration(days: 7),
);

const updateItem = AlfredItem(
  title: 'Auto-Update available!',
  subtitle: 'Press <enter> to auto-update to a new version of this workflow.',
  arg: 'update:workflow',
  match:
      'Auto-Update available! Press <enter> to auto-update to a new version of this workflow.',
  icon: AlfredItemIcon(path: 'alfredhatcog.png'),
  valid: true,
);

void _showPlaceholder() {
  _workflow.addItem(
    const AlfredItem(
      title: 'Search the Laravel docs...',
      icon: AlfredItemIcon(path: 'icon.png'),
    ),
  );
}

Future<void> _performSearch(String query, {String? version}) async {
  final AlgoliaQuerySnapshot snapshot = await AlgoliaSearch.query(
    query,
    version: version,
  );

  if (snapshot.nbHits > 0) {
    final AlfredItems items = AlfredItems(
      snapshot.hits.map((snapshot) => SearchResult.fromJson(snapshot.data)).map(
        (result) {
          final String title = _unescape.convert(result.hierarchy.last);
          final Map<String, String?> hierarchy = result.hierarchy.toJson()
            ..removeWhere((_, value) => value == null || value == title);

          return AlfredItem(
            uid: result.objectID,
            title: title,
            subtitle: hierarchy.length > 0
                ? _unescape.convert(hierarchy.values.join(' > ')).truncate(75)
                : '',
            arg: result.url,
            text: AlfredItemText(
              largeType: title,
              copy: result.url,
            ),
            quickLookUrl: result.url,
            icon: AlfredItemIcon(path: 'icon.png'),
            valid: true,
          );
        },
      ).toList(),
    );
    _workflow.addItems(items.items);
  } else {
    final Uri url =
        Uri.https('www.google.com', '/search', {'q': 'Laravel $query'});

    _workflow.addItem(
      AlfredItem(
        title: 'No matching answers found',
        subtitle: 'Shall I try and search Google?',
        arg: url.toString(),
        text: AlfredItemText(
          copy: url.toString(),
        ),
        quickLookUrl: url.toString(),
        icon: AlfredItemIcon(path: 'google.png'),
        valid: true,
      ),
    );
  }
}