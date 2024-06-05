import 'package:flutter/widgets.dart';
import 'package:flutter_map/flutter_map.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:latlong2/latlong.dart';
import 'map_tile_provider.dart';

class BeeLiveMap extends ConsumerWidget {
  final MapOptions options;
  final MapController controller;
  final List<Polygon>? polygons;
  final List<Polyline>? lines;

  BeeLiveMap({
    super.key,
    final MapController? mapController,
    this.polygons,
    this.lines,
    this.options = _defaultOptions,
  }) : controller = mapController ?? MapController();

  @override
  Widget build(final BuildContext context, final WidgetRef ref) {
    final tileProvider = ref.watch(tileLayerProvider);
    final polygons = this.polygons;
    final lines = this.lines;

    return FlutterMap(
      mapController: controller,
      options: options,
      children: [
        tileProvider,
        if (polygons != null)
          PolygonLayer(
            polygons: polygons,
            polygonCulling: true,
          ),
        if (lines != null)
          PolylineLayer(
            polylines: lines,
            polylineCulling: true,
          ),
      ],
    );
  }
}

const trentoCoords = LatLng(46.07143, 11.12052);

const _defaultOptions = MapOptions(
  initialCenter: trentoCoords,
  initialZoom: 14,
  maxZoom: 19,
  minZoom: 11,
  interactionOptions: InteractionOptions(
    enableScrollWheel: true,
  ),
);
