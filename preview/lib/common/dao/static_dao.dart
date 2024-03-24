
import 'package:beelive/common/dao/dao.dart';
import 'package:beelive/common/map/polygons.dart' as polygons;
import 'package:beelive/common/model/nullable_datetime_range.dart';
import 'package:beelive/common/problem.dart';

class StaticDao implements Dao {
  const StaticDao();

  @override
  List<Problem> problems() {
    return [
      Problem(
        title: "Incontro G7",
        summary: "Modifiche al trasporto pubblico.",
        validity: NullableDateTimeRange(
          begin: DateTime(2024, 3, 14, 22),
          end: DateTime(2024, 3, 15, 19),
        ),
        visibility: NullableDateTimeRange(end: DateTime(2024, 3, 15, 19)),
        categories: [Category.publicTransport],
      ),
      Problem(
        title: "Incontro G7",
        summary: "Divieti di fermata e transito.",
        validity: NullableDateTimeRange(
          begin: DateTime(2024, 3, 14, 22),
          end: DateTime(2024, 3, 15, 19),
        ),
        visibility: NullableDateTimeRange(end: DateTime(2024, 3, 15, 19)),
        categories: [Category.viability],
        events: [
          Event(
            title: "Overview",
            description: "",
            validity: NullableDateTimeRange(
              begin: DateTime(2024, 3, 14, 22),
              end: DateTime(2024, 3, 15, 19),
            ),
          ),
          Event(
            title: "Fase preparatoria",
            description: "",
            validity: NullableDateTimeRange(
              begin: DateTime(2024, 3, 14, 22),
              end: DateTime(2024, 3, 15, 19),
            ),
          ),
          Event(
            title: "Arrivo delegazioni",
            description:
                "In occasione della Riunione ministeriale Industria, Tecnologia e Digitale del G7, in programma la prossima settimana a palazzo Geremia, palazzo Thun e nella sede della Provincia di piazza Dante, sono previste alcune limitazioni alla circolazione e alcune deviazioni che interesseranno sia la tangenziale sia le vie del centro storico.\nIn un’area ristretta del centro città saranno rimossi anche i plateatici e vietata la sosta e, nel momento clou della manifestazione, pure il transito pedonale potrà subire alcune limitazioni.\n\nA chi, venerdì 15 marzo, avesse la necessità di raggiungere Trento in auto, il consiglio della polizia locale è quello di arrivare entro le 8, in modo da non incappare nelle limitazioni al traffico. In alternativa, si suggerisce di privilegiare l’uso dei mezzi pubblici.\n\nNei momenti clou dell’evento saranno chiuse alcune uscite della tangenziale. Saranno vietati l’accesso e la sosta nella zona del centro storico intorno a via Belenzani (da piazza Duomo a via Romagnosi).\nNella fascia oraria di arrivo e partenza delle delegazioni sono previste restrizioni al transito pedonale.\n\nPer informazioni sulla viabilità da lunedì si potrà chiamare il numero dedicato della polizia locale 0461 889400 o scrivere a poliziam.serviziesterni@comune.trento.it.",
            validity: NullableDateTimeRange(
              begin: DateTime(2024, 3, 14, 22),
              end: DateTime(2024, 3, 15, 19),
            ),
            polygons: [polygons.g7_1, polygons.g7_2, polygons.g7_3],
          ),
          Event(
            title: "Evento in corso",
            description: "",
            validity: NullableDateTimeRange(
              begin: DateTime(2024, 3, 14, 22),
              end: DateTime(2024, 3, 15, 19),
            ),
          ),
          Event(
            title: "Partenza delle delegazioni",
            description: "",
            validity: NullableDateTimeRange(
              begin: DateTime(2024, 3, 14, 22),
              end: DateTime(2024, 3, 15, 19),
            ),
          ),
        ],
      ),
      Problem(
        title: "Parcheggio Via Canestrini",
        summary: "Cambio della destinazione d'uso.",
        validity: NullableDateTimeRange(begin: DateTime(2023, 11, 6)),
        visibility: NullableDateTimeRange(begin: DateTime(2023, 11, 6)),
        categories: [Category.parking],
        events: [
          Event(
            title: "",
            validity: NullableDateTimeRange(begin: DateTime(2023, 11, 6)),
            polygons: [
              polygons.parcheggioCanestrini
            ]
          ),
        ],
      )
    ];
  }
}
