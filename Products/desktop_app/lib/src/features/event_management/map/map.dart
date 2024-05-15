import 'package:flutter_map/flutter_map.dart';
import 'package:latlong2/latlong.dart';

class BeeLiveMap extends FlutterMap {
  const BeeLiveMap({
    super.key,
    required super.children,
    super.mapController,
    super.options = _mapOptions,
  });

  static const _mapOptions = MapOptions(
    initialCenter: LatLng(46.07143, 11.12052),
    initialZoom: 14,
    maxZoom: 19,
    minZoom: 11,
    interactionOptions: InteractionOptions(
      enableScrollWheel: true,
    ),
  );
}
