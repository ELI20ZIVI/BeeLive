

import 'package:flutter_map/flutter_map.dart';
import 'package:flutter_map_cancellable_tile_provider/flutter_map_cancellable_tile_provider.dart';

/// The tile server provider used for the BeeLive map.
/// TODO: check licenses.
TileLayer get openStreetMapTileLayer => TileLayer(
  // urlTemplate: 'https://tile.openstreetmap.org/{z}/{x}/{y}.png',
  // urlTemplate: 'https://{s}.tile.openstreetmap.fr/hot/{z}/{x}/{y}.png',
  urlTemplate:
  'https://server.arcgisonline.com/ArcGIS/rest/services/World_Topo_Map/MapServer/tile/{z}/{y}/{x}',
  userAgentPackageName: 'beelive',
  tileProvider: CancellableNetworkTileProvider(),
);
