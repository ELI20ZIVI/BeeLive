


import 'package:flutter/foundation.dart';
import 'package:flutter_map/flutter_map.dart';
import 'package:flutter_map_cancellable_tile_provider/flutter_map_cancellable_tile_provider.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

StateProvider<Uri> tileLayerUriProvider = StateProvider((ref) {
  return Uri(
    scheme: "https",
    host: "server.arcgisonline.com",
    path: "ArcGIS/rest/services/World_Topo_Map/MapServer/tile",
  );
});

AutoDisposeProvider<TileLayer> tileLayerProvider = Provider.autoDispose((ref) {
  final uri = ref.watch(tileLayerUriProvider);
  debugPrint(uri.toString());

  return TileLayer(
    urlTemplate: '$uri/{z}/{y}/{x}',
    userAgentPackageName: 'beelive',
    tileProvider: CancellableNetworkTileProvider(),
  );

});
