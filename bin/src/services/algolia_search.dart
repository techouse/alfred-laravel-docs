import 'package:algolia/algolia.dart'
    show Algolia, AlgoliaQuery, AlgoliaQuerySnapshot;

import '../env/env.dart';
import '../models/search_result.dart';

class AlgoliaSearch {
  AlgoliaSearch._();

  static final Algolia _algolia = Algolia.init(
    applicationId: Env.algoliaApplicationId,
    apiKey: Env.algoliaSearchOnlyApiKey,
  );

  static Future<AlgoliaQuerySnapshot> query(
    String queryString, {
    String? version,
  }) async {
    final AlgoliaQuery query = _algolia.instance
        .index(Env.algoliaSearchIndex)
        .query(queryString)
        .facetFilter(
          'version:${version ?? Env.supportedVersions.values.last}',
        )
        .setAttributesToRetrieve(SearchResult.attributesToRetrieve)
        .setPage(0)
        .setHitsPerPage(9);

    return await query.getObjects();
  }
}
