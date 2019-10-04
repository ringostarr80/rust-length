extern crate length;

use length::{AstronomicUnit::*, ImperialUnit::*, Length, MetricUnit::*, Unit};

#[test]
fn test_new() {
    let distance = Length::new();
    assert_eq!(distance.unit, Unit::Metric(Meter));
    assert_eq!(distance.value, 0.0);
}

#[test]
fn test_new_value_unit() {
    let distance = Length::new_value_unit(2.5, Unit::Metric(Kilometer));
    assert_eq!(distance.unit, Unit::Metric(Kilometer));
    assert_eq!(distance.value, 2.5);
}

#[test]
fn test_default() {
    let distance = Length::default();
    assert_eq!(distance.unit, Unit::Metric(Meter));
    assert_eq!(distance.value, 0.0);
}

#[test]
fn test_to_string_for_metrics() {
    let ym = Length::new_value_unit(10, Unit::Metric(Yoctometer));
    assert_eq!(ym.to_string(), "10 ym");

    let zm = Length::new_value_unit(10, Unit::Metric(Zeptometer));
    assert_eq!(zm.to_string(), "10 zm");

    let am = Length::new_value_unit(10, Unit::Metric(Attometer));
    assert_eq!(am.to_string(), "10 am");

    let fm = Length::new_value_unit(10, Unit::Metric(Femtometer));
    assert_eq!(fm.to_string(), "10 fm");

    let pm = Length::new_value_unit(10, Unit::Metric(Picometer));
    assert_eq!(pm.to_string(), "10 pm");

    let nm = Length::new_value_unit(10, Unit::Metric(Nanometer));
    assert_eq!(nm.to_string(), "10 nm");

    let microm = Length::new_value_unit(10, Unit::Metric(Micrometer));
    assert_eq!(microm.to_string(), "10 µm");

    let mm = Length::new_value_unit(10, Unit::Metric(Millimeter));
    assert_eq!(mm.to_string(), "10 mm");

    let cm = Length::new_value_unit(1.5, Unit::Metric(Centimeter));
    assert_eq!(cm.to_string(), "1.5 cm");

    let dm = Length::new_value_unit(45.12, Unit::Metric(Decimeter));
    assert_eq!(dm.to_string(), "45.12 dm");

    let m = Length::new_value_unit(100, Unit::Metric(Meter));
    assert_eq!(m.to_string(), "100 m");

    let dam = Length::new_value_unit(100, Unit::Metric(Decameter));
    assert_eq!(dam.to_string(), "100 dam");

    let hm = Length::new_value_unit(100, Unit::Metric(Hectometer));
    assert_eq!(hm.to_string(), "100 hm");

    let km = Length::new_value_unit(0.5, Unit::Metric(Kilometer));
    assert_eq!(km.to_string(), "0.5 km");

    let megam = Length::new_value_unit(0.5, Unit::Metric(Megameter));
    assert_eq!(megam.to_string(), "0.5 Mm");

    let gigam = Length::new_value_unit(0.5, Unit::Metric(Gigameter));
    assert_eq!(gigam.to_string(), "0.5 Gm");

    let teram = Length::new_value_unit(0.5, Unit::Metric(Terameter));
    assert_eq!(teram.to_string(), "0.5 Tm");

    let petam = Length::new_value_unit(0.5, Unit::Metric(Petameter));
    assert_eq!(petam.to_string(), "0.5 Pm");

    let exam = Length::new_value_unit(0.5, Unit::Metric(Exameter));
    assert_eq!(exam.to_string(), "0.5 Em");

    let zettam = Length::new_value_unit(0.5, Unit::Metric(Zettameter));
    assert_eq!(zettam.to_string(), "0.5 Zm");

    let yottam = Length::new_value_unit(0.5, Unit::Metric(Yottameter));
    assert_eq!(yottam.to_string(), "0.5 Ym");
}

#[test]
fn test_to_string_for_imperials() {
    let inch = Length::new_value_unit(10, Unit::Imperial(Inch));
    assert_eq!(inch.to_string(), "10 in");

    let foot = Length::new_value_unit(10, Unit::Imperial(Foot));
    assert_eq!(foot.to_string(), "10 ft");

    let yard = Length::new_value_unit(10, Unit::Imperial(Yard));
    assert_eq!(yard.to_string(), "10 yd");

    let mile = Length::new_value_unit(10, Unit::Imperial(Mile));
    assert_eq!(mile.to_string(), "10 mi");
}

#[test]
fn test_to_string_for_astronomics() {
    let au = Length::new_value_unit(2, Unit::Astronomic(AstronomicalUnit));
    assert_eq!(au.to_string(), "2 au");

    let ly = Length::new_value_unit(2, Unit::Astronomic(Lightyear));
    assert_eq!(ly.to_string(), "2 ly");

    let pc = Length::new_value_unit(3.5, Unit::Astronomic(Parsec));
    assert_eq!(pc.to_string(), "3.5 pc");
}

#[test]
fn test_from_mm_to_x() {
    let one_mm = Length::new_value_unit(1, Unit::Metric(Millimeter));

    let mm_to_mm = one_mm.to(Unit::Metric(Millimeter));
    assert_eq!(mm_to_mm.unit, Unit::Metric(Millimeter));
    assert_eq!(mm_to_mm.value, 1.0);

    let mm_to_cm = one_mm.to(Unit::Metric(Centimeter));
    assert_eq!(mm_to_cm.unit, Unit::Metric(Centimeter));
    assert_eq!(mm_to_cm.value, 0.1);

    let mm_to_dm = one_mm.to(Unit::Metric(Decimeter));
    assert_eq!(mm_to_dm.unit, Unit::Metric(Decimeter));
    assert_eq!(mm_to_dm.value, 0.01);

    let mm_to_m = one_mm.to(Unit::Metric(Meter));
    assert_eq!(mm_to_m.unit, Unit::Metric(Meter));
    assert_eq!(mm_to_m.value, 0.001);

    let mm_to_km = one_mm.to(Unit::Metric(Kilometer));
    assert_eq!(mm_to_km.unit, Unit::Metric(Kilometer));
    assert_eq!(mm_to_km.value, 0.000001);
}

#[test]
fn test_from_cm_to_x() {
    let one_cm = Length::new_value_unit(1, Unit::Metric(Centimeter));

    let cm_to_mm = one_cm.to(Unit::Metric(Millimeter));
    assert_eq!(cm_to_mm.unit, Unit::Metric(Millimeter));
    assert_eq!(cm_to_mm.value, 10.0);

    let cm_to_cm = one_cm.to(Unit::Metric(Centimeter));
    assert_eq!(cm_to_cm.unit, Unit::Metric(Centimeter));
    assert_eq!(cm_to_cm.value, 1.0);

    let cm_to_dm = one_cm.to(Unit::Metric(Decimeter));
    assert_eq!(cm_to_dm.unit, Unit::Metric(Decimeter));
    assert_eq!(cm_to_dm.value, 0.1);

    let cm_to_m = one_cm.to(Unit::Metric(Meter));
    assert_eq!(cm_to_m.unit, Unit::Metric(Meter));
    assert_eq!(cm_to_m.value, 0.01);

    let cm_to_km = one_cm.to(Unit::Metric(Kilometer));
    assert_eq!(cm_to_km.unit, Unit::Metric(Kilometer));
    assert_eq!(cm_to_km.value, 0.00001);
}

#[test]
fn test_from_dm_to_x() {
    let one_dm = Length::new_value_unit(1, Unit::Metric(Decimeter));

    let dm_to_mm = one_dm.to(Unit::Metric(Millimeter));
    assert_eq!(dm_to_mm.unit, Unit::Metric(Millimeter));
    assert_eq!(dm_to_mm.value, 100.0);

    let dm_to_cm = one_dm.to(Unit::Metric(Centimeter));
    assert_eq!(dm_to_cm.unit, Unit::Metric(Centimeter));
    assert_eq!(dm_to_cm.value, 10.0);

    let dm_to_dm = one_dm.to(Unit::Metric(Decimeter));
    assert_eq!(dm_to_dm.unit, Unit::Metric(Decimeter));
    assert_eq!(dm_to_dm.value, 1.0);

    let dm_to_m = one_dm.to(Unit::Metric(Meter));
    assert_eq!(dm_to_m.unit, Unit::Metric(Meter));
    assert_eq!(dm_to_m.value, 0.1);

    let dm_to_km = one_dm.to(Unit::Metric(Kilometer));
    assert_eq!(dm_to_km.unit, Unit::Metric(Kilometer));
    assert_eq!(dm_to_km.value, 0.0001);
}

#[test]
fn test_from_m_to_x() {
    let one_m = Length::new_value_unit(1, Unit::Metric(Meter));

    let m_to_mm = one_m.to(Unit::Metric(Millimeter));
    assert_eq!(m_to_mm.unit, Unit::Metric(Millimeter));
    assert_eq!(m_to_mm.value, 1_000.0);

    let m_to_cm = one_m.to(Unit::Metric(Centimeter));
    assert_eq!(m_to_cm.unit, Unit::Metric(Centimeter));
    assert_eq!(m_to_cm.value, 100.0);

    let m_to_dm = one_m.to(Unit::Metric(Decimeter));
    assert_eq!(m_to_dm.unit, Unit::Metric(Decimeter));
    assert_eq!(m_to_dm.value, 10.0);

    let m_to_m = one_m.to(Unit::Metric(Meter));
    assert_eq!(m_to_m.unit, Unit::Metric(Meter));
    assert_eq!(m_to_m.value, 1.0);

    let m_to_km = one_m.to(Unit::Metric(Kilometer));
    assert_eq!(m_to_km.unit, Unit::Metric(Kilometer));
    assert_eq!(m_to_km.value, 0.001);
}

#[test]
fn test_from_km_to_x() {
    let one_km = Length::new_value_unit(1, Unit::Metric(Kilometer));

    let km_to_mm = one_km.to(Unit::Metric(Millimeter));
    assert_eq!(km_to_mm.unit, Unit::Metric(Millimeter));
    assert_eq!(km_to_mm.value, 1_000_000.0);

    let km_to_cm = one_km.to(Unit::Metric(Centimeter));
    assert_eq!(km_to_cm.unit, Unit::Metric(Centimeter));
    assert_eq!(km_to_cm.value, 100_000.0);

    let km_to_dm = one_km.to(Unit::Metric(Decimeter));
    assert_eq!(km_to_dm.unit, Unit::Metric(Decimeter));
    assert_eq!(km_to_dm.value, 10_000.0);

    let km_to_m = one_km.to(Unit::Metric(Meter));
    assert_eq!(km_to_m.unit, Unit::Metric(Meter));
    assert_eq!(km_to_m.value, 1_000.0);

    let km_to_km = one_km.to(Unit::Metric(Kilometer));
    assert_eq!(km_to_km.unit, Unit::Metric(Kilometer));
    assert_eq!(km_to_km.value, 1.0);
}

#[test]
fn test_inch_to_x() {
    let inch = Length::new_value_unit(1, Unit::Imperial(Inch));

    let inch_to_inch = inch.to(Unit::Imperial(Inch));
    assert_eq!(inch_to_inch.unit, Unit::Imperial(Inch));
    assert_eq!(inch_to_inch.value, 1.0);

    let inch_to_foot = inch.to(Unit::Imperial(Foot));
    assert_eq!(inch_to_foot.unit, Unit::Imperial(Foot));
    assert_eq!(inch_to_foot.value, 1.0 / 12.0);

    let inch_to_yard = inch.to(Unit::Imperial(Yard));
    assert_eq!(inch_to_yard.unit, Unit::Imperial(Yard));
    assert_eq!(inch_to_yard.value, 1.0 / 36.0);

    let inch_to_mile = inch.to(Unit::Imperial(Mile));
    assert_eq!(inch_to_mile.unit, Unit::Imperial(Mile));
    assert_eq!(inch_to_mile.value, 1.0 / 63360.0);
}

#[test]
fn test_foot_to_x() {
    let foot = Length::new_value_unit(1, Unit::Imperial(Foot));

    let foot_to_inch = foot.to(Unit::Imperial(Inch));
    assert_eq!(foot_to_inch.unit, Unit::Imperial(Inch));
    assert_eq!(foot_to_inch.value, 12.0);

    let foot_to_foot = foot.to(Unit::Imperial(Foot));
    assert_eq!(foot_to_foot.unit, Unit::Imperial(Foot));
    assert_eq!(foot_to_foot.value, 1.0);

    let foot_to_yard = foot.to(Unit::Imperial(Yard));
    assert_eq!(foot_to_yard.unit, Unit::Imperial(Yard));
    assert_eq!(foot_to_yard.value, 1.0 / 3.0);

    let foot_to_mile = foot.to(Unit::Imperial(Mile));
    assert_eq!(foot_to_mile.unit, Unit::Imperial(Mile));
    assert_eq!(foot_to_mile.value, 1.0 / 5280.0);
}

#[test]
fn test_yard_to_x() {
    let yard = Length::new_value_unit(1, Unit::Imperial(Yard));

    let yard_to_inch = yard.to(Unit::Imperial(Inch));
    assert_eq!(yard_to_inch.unit, Unit::Imperial(Inch));
    assert_eq!(yard_to_inch.value, 36.0);

    let yard_to_foot = yard.to(Unit::Imperial(Foot));
    assert_eq!(yard_to_foot.unit, Unit::Imperial(Foot));
    assert_eq!(yard_to_foot.value, 3.0);

    let yard_to_yard = yard.to(Unit::Imperial(Yard));
    assert_eq!(yard_to_yard.unit, Unit::Imperial(Yard));
    assert_eq!(yard_to_yard.value, 1.0);

    let yard_to_mile = yard.to(Unit::Imperial(Mile));
    assert_eq!(yard_to_mile.unit, Unit::Imperial(Mile));
    assert_eq!(yard_to_mile.value, 1.0 / 1760.0);
}

#[test]
fn test_mile_to_x() {
    let mile = Length::new_value_unit(1, Unit::Imperial(Mile));

    let mile_to_inch = mile.to(Unit::Imperial(Inch));
    assert_eq!(mile_to_inch.unit, Unit::Imperial(Inch));
    assert_eq!(mile_to_inch.value, 63360.0);

    let mile_to_foot = mile.to(Unit::Imperial(Foot));
    assert_eq!(mile_to_foot.unit, Unit::Imperial(Foot));
    assert_eq!(mile_to_foot.value, 5280.0);

    let mile_to_yard = mile.to(Unit::Imperial(Yard));
    assert_eq!(mile_to_yard.unit, Unit::Imperial(Yard));
    assert_eq!(mile_to_yard.value, 1760.0);

    let mile_to_mile = mile.to(Unit::Imperial(Mile));
    assert_eq!(mile_to_mile.unit, Unit::Imperial(Mile));
    assert_eq!(mile_to_mile.value, 1.0);
}

#[test]
fn test_au_to_x() {
    let au = Length::new_value_unit(1, Unit::Astronomic(AstronomicalUnit));

    let au_to_ly = au.to(Unit::Astronomic(Lightyear));
    assert_eq!(au_to_ly.unit, Unit::Astronomic(Lightyear));
    assert_eq!(au_to_ly.value, 0.000_015_812_507_409_820_66);

    let au_to_au = au.to(Unit::Astronomic(AstronomicalUnit));
    assert_eq!(au_to_au.unit, Unit::Astronomic(AstronomicalUnit));
    assert_eq!(au_to_au.value, 1.0);

    let au_to_pc = au.to(Unit::Astronomic(Parsec));
    assert_eq!(au_to_pc.unit, Unit::Astronomic(Parsec));
    assert_eq!(au_to_pc.value, 0.000_004_848_136_811_095_361);
}

#[test]
fn test_ly_to_x() {
    let ly = Length::new_value_unit(1, Unit::Astronomic(Lightyear));

    let ly_to_ld = ly.to(Unit::Astronomic(Lightday));
    assert_eq!(ly_to_ld.unit, Unit::Astronomic(Lightday));
    assert_eq!(ly_to_ld.value, 365.25);

    let ly_to_lh = ly.to(Unit::Astronomic(Lighthour));
    assert_eq!(ly_to_lh.unit, Unit::Astronomic(Lighthour));
    assert_eq!(ly_to_lh.value, 365.25 * 24.0);

    let ly_to_lm = ly.to(Unit::Astronomic(Lightminute));
    assert_eq!(ly_to_lm.unit, Unit::Astronomic(Lightminute));
    assert_eq!(ly_to_lm.value, 365.25 * 24.0 * 60.0);

    let ly_to_ls = ly.to(Unit::Astronomic(Lightsecond));
    assert_eq!(ly_to_ls.unit, Unit::Astronomic(Lightsecond));
    assert_eq!(ly_to_ls.value, 31_557_600.000_000_004); // 365.25 * 24.0 * 60.0 * 60.0

    let ly_to_au = ly.to(Unit::Astronomic(AstronomicalUnit));
    assert_eq!(ly_to_au.unit, Unit::Astronomic(AstronomicalUnit));
    assert_eq!(ly_to_au.value, 63_241.077_084_266_275);

    let ly_to_ly = ly.to(Unit::Astronomic(Lightyear));
    assert_eq!(ly_to_ly.unit, Unit::Astronomic(Lightyear));
    assert_eq!(ly_to_ly.value, 1.0);

    let ly_to_pc = ly.to(Unit::Astronomic(Parsec));
    assert_eq!(ly_to_pc.unit, Unit::Astronomic(Parsec));
    assert_eq!(ly_to_pc.value, 0.306_601_393_785_550_57);
}

#[test]
fn test_pc_to_x() {
    let pc = Length::new_value_unit(1, Unit::Astronomic(Parsec));

    let pc_to_au = pc.to(Unit::Astronomic(AstronomicalUnit));
    assert_eq!(pc_to_au.unit, Unit::Astronomic(AstronomicalUnit));
    assert_eq!(pc_to_au.value, 206_264.806_247_096_36);

    let pc_to_ly = pc.to(Unit::Astronomic(Lightyear));
    assert_eq!(pc_to_ly.unit, Unit::Astronomic(Lightyear));
    assert_eq!(pc_to_ly.value, 3.261_563_777_167_433_7);

    let pc_to_pc = pc.to(Unit::Astronomic(Parsec));
    assert_eq!(pc_to_pc.unit, Unit::Astronomic(Parsec));
    assert_eq!(pc_to_pc.value, 1.0);
}

#[test]
fn test_metric_to_imperial() {
    let km = Length::new_value_unit(1, Unit::Metric(Kilometer));

    let km_to_mile = km.to(Unit::Imperial(Mile));
    assert_eq!(km_to_mile.unit, Unit::Imperial(Mile));
    assert_eq!(km_to_mile.value, 0.621371192237334);

    let km_to_yard = km.to(Unit::Imperial(Yard));
    assert_eq!(km_to_yard.unit, Unit::Imperial(Yard));
    assert_eq!(km_to_yard.value, 1_093.613_298_337_707_9);
}

#[test]
fn test_metric_to_astronomic() {
    let km = Length::new_value_unit(9_460_730_472_580.8, Unit::Metric(Kilometer));

    let km_to_au = km.to(Unit::Astronomic(AstronomicalUnit));
    assert_eq!(km_to_au.unit, Unit::Astronomic(AstronomicalUnit));
    assert_eq!(km_to_au.value, 63_241.077_084_266_275);

    let km_to_ly = km.to(Unit::Astronomic(Lightyear));
    assert_eq!(km_to_ly.unit, Unit::Astronomic(Lightyear));
    assert_eq!(km_to_ly.value, 1.0);
}

#[test]
fn test_imperial_to_metric() {
    let mile = Length::new_value_unit(1, Unit::Imperial(Mile));

    let mile_to_km = mile.to(Unit::Metric(Kilometer));
    assert_eq!(mile_to_km.unit, Unit::Metric(Kilometer));
    assert_eq!(mile_to_km.value, 1.609344);

    let inch = Length::new_value_unit(1, Unit::Imperial(Inch));

    let inch_to_cm = inch.to(Unit::Metric(Centimeter));
    assert_eq!(inch_to_cm.unit, Unit::Metric(Centimeter));
    assert_eq!(inch_to_cm.value, 2.54);
}

#[test]
fn test_imperial_to_astronomic() {
    let mi = Length::new_value_unit(5_878_625_373_183.607, Unit::Imperial(Mile));

    let mi_to_au = mi.to(Unit::Astronomic(AstronomicalUnit));
    assert_eq!(mi_to_au.unit, Unit::Astronomic(AstronomicalUnit));
    assert_eq!(mi_to_au.value, 63_241.077_084_266_275);

    let mi_to_ly = mi.to(Unit::Astronomic(Lightyear));
    assert_eq!(mi_to_ly.unit, Unit::Astronomic(Lightyear));
    assert_eq!(mi_to_ly.value, 1.0);
}

#[test]
fn test_astronomic_to_metric() {
    let ly = Length::new_value_unit(1, Unit::Astronomic(Lightyear));

    let ly_to_km = ly.to(Unit::Metric(Kilometer));
    assert_eq!(ly_to_km.unit, Unit::Metric(Kilometer));
    assert_eq!(ly_to_km.value, 9_460_730_472_580.8);

    let au = Length::new_value_unit(1, Unit::Astronomic(AstronomicalUnit));

    let au_to_m = au.to(Unit::Metric(Meter));
    assert_eq!(au_to_m.unit, Unit::Metric(Meter));
    assert_eq!(au_to_m.value, 149_597_870_700.0);
}

#[test]
fn test_astronomic_to_imperial() {
    let ly = Length::new_value_unit(1, Unit::Astronomic(Lightyear));

    let ly_to_mi = ly.to(Unit::Imperial(Mile));
    assert_eq!(ly_to_mi.unit, Unit::Imperial(Mile));
    assert_eq!(ly_to_mi.value, 5_878_625_373_183.607);
}

#[test]
fn test_new_string() {
    let one_m = Length::new_string("1m").unwrap();
    assert_eq!(one_m.unit, Unit::Metric(Meter));
    assert_eq!(one_m.value, 1.0);

    let km_test = Length::new_string("23.5 km").unwrap();
    assert_eq!(km_test.unit, Unit::Metric(Kilometer));
    assert_eq!(km_test.value, 23.5);

    let ly_test = Length::new_string("2.3 ly").unwrap();
    assert_eq!(ly_test.unit, Unit::Astronomic(Lightyear));
    assert_eq!(ly_test.value, 2.3);
}

#[test]
fn test_get_original_string() {
    let one_m = Length::new_string("1m").unwrap();
    assert_eq!(one_m.get_original_string(), "1m");

    let km_test = Length::new_string("23.5 km").unwrap();
    assert_eq!(km_test.get_original_string(), "23.5 km");

    let ly_test = Length::new_string("2.3 ly").unwrap();
    assert_eq!(ly_test.get_original_string(), "2.3 ly");
}
