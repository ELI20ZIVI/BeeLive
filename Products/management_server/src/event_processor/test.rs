#[cfg(test)]
mod tests {
    use std::iter;
    use chrono::{Local, TimeZone, Utc};
    use geo::polygon;
    use geojson::{Feature, FeatureCollection, Geometry, Value};
    use crate::dao::objects::{NullableDateTimeRange, RiskLevel, SubEvent};
    use crate::dao::objects::{Event};
    use crate::event_processor;

    #[test]
    fn it_works() {
        let mut event = Event{
            id: 0,
            title: "Incontro G7".to_string(),
            summary: "Divieti di fermata e transito.".to_string(),
            description: String::default(),
            creator_id: 0,
            risk_level: RiskLevel::Warning,
            polygons: FeatureCollection::from_iter(iter::empty()),
            validity: NullableDateTimeRange::new(
                Utc.with_ymd_and_hms(2024, 3, 14, 22, 0, 0).single(),
                Utc.with_ymd_and_hms(2024, 3, 15, 19, 0, 0).single()
            ),
            visibility: NullableDateTimeRange::new(
                None,
                Utc.with_ymd_and_hms(2024, 3, 15, 19, 0, 0).single()
            ),
            categories: vec![],
            subevents: vec![
                SubEvent{
                    title: "Fase preparatoria".to_string(),
                    description: String::default(),
                    polygons: FeatureCollection::from_iter(iter::empty()),
                    validity: NullableDateTimeRange::new(
                        Utc.with_ymd_and_hms(2024, 3, 14, 22, 0, 0).single(),
                        Utc.with_ymd_and_hms(2024, 3, 15, 19, 0, 0).single(),
                    ),
                },
                SubEvent{
                    title: "Arrivo delegazioni".to_string(),
                    description: "In occasione della Riunione ministeriale Industria, Tecnologia e Digitale del G7, in programma la prossima settimana a palazzo Geremia, palazzo Thun e nella sede della Provincia di piazza Dante, sono previste alcune limitazioni alla circolazione e alcune deviazioni che interesseranno sia la tangenziale sia le vie del centro storico.\nIn un’area ristretta del centro città saranno rimossi anche i plateatici e vietata la sosta e, nel momento clou della manifestazione, pure il transito pedonale potrà subire alcune limitazioni.\n\nA chi, venerdì 15 marzo, avesse la necessità di raggiungere Trento in auto, il consiglio della polizia locale è quello di arrivare entro le 8, in modo da non incappare nelle limitazioni al traffico. In alternativa, si suggerisce di privilegiare l’uso dei mezzi pubblici.\n\nNei momenti clou dell’evento saranno chiuse alcune uscite della tangenziale. Saranno vietati l’accesso e la sosta nella zona del centro storico intorno a via Belenzani (da piazza Duomo a via Romagnosi).\nNella fascia oraria di arrivo e partenza delle delegazioni sono previste restrizioni al transito pedonale.\n\nPer informazioni sulla viabilità da lunedì si potrà chiamare il numero dedicato della polizia locale 0461 889400 o scrivere a poliziam.serviziesterni@comune.trento.it.".to_string(),
                    validity: NullableDateTimeRange::new(
                        Utc.with_ymd_and_hms(2024, 3, 14, 22, 0, 0).single(),
                        Utc.with_ymd_and_hms(2024, 3, 15, 19, 0, 0).single(),
                    ),
                    polygons: FeatureCollection::from_iter(vec![
                        polygon![
                            geo::Coord{y: 46.06685718388318, x: 11.12094551324845},
                            geo::Coord{y: 46.06686090581562, x: 11.12103939056397},
                            geo::Coord{y: 46.06673063803123, x: 11.12101256847382},
                            geo::Coord{y: 46.06668225277588, x: 11.12125664949417},
                            geo::Coord{y: 46.06682182551257, x: 11.12229466438294},
                            geo::Coord{y: 46.06727962161192, x: 11.12214714288712},
                            geo::Coord{y: 46.06732614539322, x: 11.12213373184204},
                            geo::Coord{y: 46.06731497968931, x: 11.12238317728043},
                            geo::Coord{y: 46.06732614539322, x: 11.12270772457123},
                            geo::Coord{y: 46.06735033774401, x: 11.12326294183731},
                            geo::Coord{y: 46.06738011293044, x: 11.12378597259522},
                            geo::Coord{y: 46.06825847370415, x: 11.1235123872757},
                            geo::Coord{y: 46.06829941390777, x: 11.12379133701325},
                            geo::Coord{y: 46.06837757239397, x: 11.12377524375916},
                            geo::Coord{y: 46.06844456529409, x: 11.12423658370972},
                            geo::Coord{y: 46.0689246786988, x: 11.12428486347199},
                            geo::Coord{y: 46.06955365815785, x: 11.12451016902924},
                            geo::Coord{y: 46.07006725737063, x: 11.12443506717682},
                            geo::Coord{y: 46.07011936137159, x: 11.12434923648834},
                            geo::Coord{y: 46.06995560577441, x: 11.12379133701325},
                            geo::Coord{y: 46.06988489297997, x: 11.12253606319428},
                            geo::Coord{y: 46.07044687110845, x: 11.122305393219},
                            geo::Coord{y: 46.07094929704095, x: 11.12357139587403},
                            geo::Coord{y: 46.07125447204132, x: 11.12412393093109},
                            geo::Coord{y: 46.07206206115715, x: 11.12156510353089},
                            geo::Coord{y: 46.07511739507579, x: 11.12361967563629},
                            geo::Coord{y: 46.07519182253701, x: 11.12353920936585},
                            geo::Coord{y: 46.07317852434455, x: 11.12215518951416},
                            geo::Coord{y: 46.07358788852097, x: 11.12074971199036},
                            geo::Coord{y: 46.07256819393402, x: 11.1200362443924},
                            geo::Coord{y: 46.07228163403479, x: 11.11995577812195},
                            geo::Coord{y: 46.07150009947451, x: 11.11947298049927},
                            geo::Coord{y: 46.07137356426665, x: 11.11944615840912},
                            geo::Coord{y: 46.07080043057478, x: 11.11955881118775},
                            geo::Coord{y: 46.07051013981091, x: 11.11965000629425},
                            geo::Coord{y: 46.07004120535172, x: 11.11973583698273},
                            geo::Coord{y: 46.06982906702563, x: 11.1180728673935},
                            geo::Coord{y: 46.06941595320864, x: 11.11838936805725},
                            geo::Coord{y: 46.06863809988211, x: 11.11858248710633},
                            geo::Coord{y: 46.06864554350656, x: 11.1187219619751},
                            geo::Coord{y: 46.06818403689248, x: 11.11873805522919},
                            geo::Coord{y: 46.06665806013228, x: 11.11905455589294},
                            geo::Coord{y: 46.06678088421305, x: 11.12012207508087},
                            geo::Coord{y: 46.06607371152021, x: 11.12025618553162},
                            geo::Coord{y: 46.06609976541174, x: 11.12039029598236},
                            geo::Coord{y: 46.06678832808796, x: 11.12024009227753},
                            geo::Coord{y: 46.06685718388318, x: 11.12094551324845},
                        ],
                        polygon![
                            geo::Coord{y: 46.06690742994984, x: 11.12096160650253},
                            geo::Coord{y: 46.06686090581562, x: 11.12103939056397},
                            geo::Coord{y: 46.06673063803123, x: 11.12101256847382},
                            geo::Coord{y: 46.06668225277588, x: 11.12125664949417},
                            geo::Coord{y: 46.06682182551257, x: 11.12229466438294},
                            geo::Coord{y: 46.06727962161192, x: 11.12214714288712},
                            geo::Coord{y: 46.06732614539322, x: 11.12213373184204},
                            geo::Coord{y: 46.06756992936624, x: 11.12208008766175},
                            geo::Coord{y: 46.06779138104972, x: 11.12212300300598},
                            geo::Coord{y: 46.06793095098202, x: 11.12244486808777},
                            geo::Coord{y: 46.06847433989027, x: 11.12263798713684},
                            geo::Coord{y: 46.06932290912955, x: 11.12251996994019},
                            geo::Coord{y: 46.06988489297997, x: 11.12253606319428},
                            geo::Coord{y: 46.07052223528989, x: 11.12225309014321},
                            geo::Coord{y: 46.0703873240281, x: 11.12158656120301},
                            geo::Coord{y: 46.0715484805067, x: 11.12120032310486},
                            geo::Coord{y: 46.07206206115715, x: 11.12156510353089},
                            geo::Coord{y: 46.0731347847003, x: 11.12228644801851},
                            geo::Coord{y: 46.07358788852097, x: 11.12074971199036},
                            geo::Coord{y: 46.07256819393402, x: 11.1200362443924},
                            geo::Coord{y: 46.07228163403479, x: 11.11995577812195},
                            geo::Coord{y: 46.07150009947451, x: 11.11947298049927},
                            geo::Coord{y: 46.07137356426665, x: 11.11944615840912},
                            geo::Coord{y: 46.07080043057478, x: 11.11955881118775},
                            geo::Coord{y: 46.07051013981091, x: 11.11965000629425},
                            geo::Coord{y: 46.07004120535172, x: 11.11973583698273},
                            geo::Coord{y: 46.06950155362286, x: 11.1197304725647},
                            geo::Coord{y: 46.06918148182892, x: 11.11986458301544},
                            geo::Coord{y: 46.06844828711949, x: 11.11998796463013},
                            geo::Coord{y: 46.06817659320581, x: 11.12036883831024},
                            geo::Coord{y: 46.06794211656136, x: 11.12062096595764},
                            geo::Coord{y: 46.06762575760548, x: 11.12084627151489},
                            geo::Coord{y: 46.06711027471683, x: 11.12099647521973},
                            geo::Coord{y: 46.06708049938486, x: 11.12092941999435},
                            geo::Coord{y: 46.06690742994984, x: 11.12096160650253},
                        ],
                        polygon!(
                            exterior: [
                                geo::Coord{y: 46.06712330141952, x: 11.12188160419464},
                                geo::Coord{y: 46.06766669827838, x: 11.12179040908814},
                                geo::Coord{y: 46.06775974514934, x: 11.12150073051453},
                                geo::Coord{y: 46.06797933514287, x: 11.12142026424408},
                                geo::Coord{y: 46.06892095690554, x: 11.12124860286713},
                                geo::Coord{y: 46.06975835406909, x: 11.12118154764175},
                                geo::Coord{y: 46.06957968102375, x: 11.11973124918834},
                                geo::Coord{y: 46.06918148182892, x: 11.11986458301544},
                                geo::Coord{y: 46.06844828711949, x: 11.11998796463013},
                                geo::Coord{y: 46.06817659320581, x: 11.12036883831024},
                                geo::Coord{y: 46.06794211656136, x: 11.12062096595764},
                                geo::Coord{y: 46.06762575760548, x: 11.12084627151489},
                                geo::Coord{y: 46.06705398071631, x: 11.12101189792156},
                                geo::Coord{y: 46.06712330141952, x: 11.12188160419464},
                            ],
                            interiors: [
                                [
                                    geo::Coord{y: 46.0696541453364, x: 11.12107157707215},
                                    geo::Coord{y: 46.06948294484847, x: 11.1198753118515},
                                    geo::Coord{y: 46.0692112560276, x: 11.11999869346619},
                                    geo::Coord{y: 46.06854877631041, x: 11.12009525299072},
                                    geo::Coord{y: 46.06809843443502, x: 11.12063705921173},
                                    geo::Coord{y: 46.06771508267084, x: 11.1209374666214},
                                    geo::Coord{y: 46.06776346702091, x: 11.12133979797364},
                                    geo::Coord{y: 46.06797933514287, x: 11.12125396728516},
                                    geo::Coord{y: 46.06937501383315, x: 11.12106084823608},
                                    geo::Coord{y: 46.0696541453364, x: 11.12107157707215},
                                ]
                            ]
                        )
                    ].into_iter().map(|g| Feature::from(Geometry::from(&g)))),
                },
                SubEvent{
                    title: "Evento in corso".to_string(),
                    description: String::default(),
                    validity: NullableDateTimeRange::new(
                        Utc.with_ymd_and_hms(2024, 3, 14, 22, 0, 0).single(),
                        Utc.with_ymd_and_hms(2024, 3, 15, 19, 0, 0).single(),
                    ),
                    polygons: FeatureCollection::from_iter(iter::empty()),
                },
                SubEvent{
                    title: "Partenza delle delegazioni".to_string(),
                    description: String::default(),
                    validity: NullableDateTimeRange::new(
                        Utc.with_ymd_and_hms(2024, 3, 14, 22, 0, 0).single(),
                        Utc.with_ymd_and_hms(2024, 3, 15, 19, 0, 0).single(),
                    ),
                    polygons: FeatureCollection::from_iter(iter::empty()),
                },
            ],
            locked_by: None,
            remote_document: Some("https://www.ufficiostampa.provincia.tn.it/Comunicati/G7-tutte-le-limitazioni-al-traffico-e-alla-sosta".to_string()),
        };

        println!("{}", event.polygons.to_string());
        event_processor::process(&mut event);
        println!("{}", event.polygons.to_string());

        let geometry = event.polygons.features[0].geometry.as_ref().unwrap();

        match geometry.value {
            Value::MultiPolygon(ref mp) => assert_eq!(mp.len(), 1),
            _ => assert!(false),
        }

    }
}