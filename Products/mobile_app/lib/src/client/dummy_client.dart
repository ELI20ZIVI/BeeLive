
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:mobile_app/src/client/client.dart';
import 'package:mobile_app/src/dtos/event.dart';
import 'package:mobile_app/src/dtos/nullable_datetime_range.dart';
import 'package:mobile_app/src/dtos/risk_level.dart';

class DummyClient implements Client {
  @override
  FutureProvider<List<Event>> get eventList => _eventList;

  final FutureProvider<List<Event>> _eventList = FutureProvider((ref) async {
    await Future.delayed(const Duration(seconds: 5));
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
            polygons: null,
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
  });
}
