
import 'dart:io';

import 'package:desktop_app/data_transfer_objects/event.dart';
import 'package:file_picker/file_picker.dart';
import 'package:fluent_ui/fluent_ui.dart';
import 'package:geojson_vi/geojson_vi.dart';
import 'package:path/path.dart';

class GeoJSONFilePicker extends StatefulWidget {

  final SubEvent subevent;

  const GeoJSONFilePicker({super.key, required this.subevent});

  @override
  State<StatefulWidget> createState() => _GeoJSONFilePickerState();

}

class _GeoJSONFilePickerState extends State<GeoJSONFilePicker> {

  String path = "";

  @override
  Widget build(BuildContext context) {

    //uso un textbox così che esce più carino
    TextBox currentPath = TextBox(placeholder: path, readOnly: true,);

    return InfoLabel(
        label: "GeoJSON File:",
        child: Row(
          children: [
            Button(
                child: const Text("Sfoglia"),
                onPressed: () async {

                  FilePickerResult? result = await FilePicker.platform.pickFiles(type: FileType.custom, allowedExtensions: ['json']);

                  if (result != null && result.paths[0] != null) {

                    String file_path = result.paths[0]!;

                    String file_content = await File(file_path).readAsString();
                    debugPrint(file_content);
                    GeoJSONFeatureCollection geojson = GeoJSONFeatureCollection.fromJSON(file_content);
                    setState(() {
                      path = basename(file_path);
                      widget.subevent.polygons = geojson;
                    });

                  }

                  debugPrint(result.toString());
                }
            ),
            const SizedBox(width: 3,),
            Expanded(child: currentPath,),
          ],
        )
    );
  }

}