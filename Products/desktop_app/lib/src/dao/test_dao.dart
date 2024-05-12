
import 'package:desktop_app/src/data_transfer_objects/category.dart';
import 'package:desktop_app/src/data_transfer_objects/risk_level.dart';
import 'package:desktop_app/src/dao/dao.dart';
import 'package:geojson_vi/geojson_vi.dart';
import 'package:mongo_dart/src/database/commands/query_and_write_operation_commands/return_classes/write_result.dart';

import '../data_transfer_objects/event.dart';
import '../data_transfer_objects/nullable_datetime_range.dart';

class TestDao implements Dao {
  const TestDao();

  @override
  List<Event> events() {
    return [
      Event(
        id: EventId(1),
        title: "Incontro G7",
        summary: "Modifiche al trasporto pubblico.",
        validity: NullableDateTimeRange(
          begin: DateTime(2024, 3, 14, 22),
          end: DateTime(2024, 3, 15, 19),
        ),
        riskLevel: RiskLevel.info,
        visibility: NullableDateTimeRange(end: DateTime(2024, 3, 15, 19)),
      ),
      Event(
        id: EventId(2),
        title: "Incontro G7",
        summary: "Divieti di fermata e transito.",
        validity: NullableDateTimeRange(
          begin: DateTime(2024, 3, 14, 22),
          end: DateTime(2024, 3, 15, 19),
        ),
        riskLevel: RiskLevel.info,
        visibility: NullableDateTimeRange(end: DateTime(2024, 3, 15, 19)),
        events: [
          SubEvent(
            title: "Overview",
            description: "",
            validity: NullableDateTimeRange(
              begin: DateTime(2024, 3, 14, 22),
              end: DateTime(2024, 3, 15, 19),
            ),
          ),
          SubEvent(
            title: "Fase preparatoria",
            description: "",
            validity: NullableDateTimeRange(
              begin: DateTime(2024, 3, 14, 22),
              end: DateTime(2024, 3, 15, 19),
            ),
          ),
          SubEvent(
            title: "Arrivo delegazioni",
            description:
            "In occasione della Riunione ministeriale Industria, Tecnologia e Digitale del G7, in programma la prossima settimana a palazzo Geremia, palazzo Thun e nella sede della Provincia di piazza Dante, sono previste alcune limitazioni alla circolazione e alcune deviazioni che interesseranno sia la tangenziale sia le vie del centro storico.\nIn un’area ristretta del centro città saranno rimossi anche i plateatici e vietata la sosta e, nel momento clou della manifestazione, pure il transito pedonale potrà subire alcune limitazioni.\n\nA chi, venerdì 15 marzo, avesse la necessità di raggiungere Trento in auto, il consiglio della polizia locale è quello di arrivare entro le 8, in modo da non incappare nelle limitazioni al traffico. In alternativa, si suggerisce di privilegiare l’uso dei mezzi pubblici.\n\nNei momenti clou dell’evento saranno chiuse alcune uscite della tangenziale. Saranno vietati l’accesso e la sosta nella zona del centro storico intorno a via Belenzani (da piazza Duomo a via Romagnosi).\nNella fascia oraria di arrivo e partenza delle delegazioni sono previste restrizioni al transito pedonale.\n\nPer informazioni sulla viabilità da lunedì si potrà chiamare il numero dedicato della polizia locale 0461 889400 o scrivere a poliziam.serviziesterni@comune.trento.it.",
            validity: NullableDateTimeRange(
              begin: DateTime(2024, 3, 14, 22),
              end: DateTime(2024, 3, 15, 19),
            ),
            polygons: GeoJSONFeatureCollection.fromJSON("""{
  "type": "FeatureCollection",
  "features": [
    {
      "type": "Feature",
      "properties": {},
      "geometry": {
        "coordinates": [
          [
            [
              11.1195557782695,
              46.06463649137629
            ],
            [
              11.126481108652513,
              46.064842424523874
            ],
            [
              11.128031063547525,
              46.0718436944706
            ],
            [
              11.11978662261538,
              46.0724842892038
            ],
            [
              11.11817071219292,
              46.06978458961078
            ],
            [
              11.108706094002656,
              46.0678169287487
            ],
            [
              11.109068849403343,
              46.06582631673794
            ],
            [
              11.119061111814034,
              46.066764430192165
            ],
            [
              11.1195557782695,
              46.06463649137629
            ]
          ]
        ],
        "type": "Polygon"
      }
    }
  ]
}""")
          ),
          SubEvent(
            title: "Evento in corso",
            description: "",
            validity: NullableDateTimeRange(
              begin: DateTime(2024, 3, 14, 22),
              end: DateTime(2024, 3, 15, 19),
            ),
          ),
          SubEvent(
            title: "Partenza delle delegazioni",
            description: "",
            validity: NullableDateTimeRange(
              begin: DateTime(2024, 3, 14, 22),
              end: DateTime(2024, 3, 15, 19),
            ),
          ),
        ],
      ),
      Event(
        id: EventId(3),
        title: "Parcheggio Via Canestrini",
        summary: "Cambio della destinazione d'uso.",
        riskLevel: RiskLevel.info,
        validity: NullableDateTimeRange(begin: DateTime(2023, 11, 6)),
        visibility: NullableDateTimeRange(begin: DateTime(2023, 11, 6)),
        events: [
          SubEvent(
            title: "",
            validity: NullableDateTimeRange(begin: DateTime(2023, 11, 6)),
            polygons: null,
          ),
        ],
      )
    ];
  }

  @override
  Future<WriteResult> insert_new_event(Event event) {
    // TODO: implement insert_new_event
    throw UnimplementedError();
  }
}
