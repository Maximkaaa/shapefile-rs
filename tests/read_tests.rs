extern crate shapefile;

use std::fs::File;
use std::io::Read;
use std::io::Cursor;
use std::io::Seek;
use std::io::SeekFrom;

const LINE_PATH: &str = "./tests/data/line.shp";
const LINEM_PATH: &str = "./tests/data/linem.shp";
const LINEZ_PATH: &str = "./tests/data/linez.shp";

const POINT_PATH: &str = "./tests/data/point.shp";
const POINTM_PATH: &str = "./tests/data/pointm.shp";
const POINTZ_PATH: &str = "./tests/data/pointz.shp";

const POLYGON_PATH: &str = "./tests/data/polygon.shp";
const POLYGONZ_PATH: &str = "./tests/data/polygonz.shp";

#[test]
fn read_line_header() {
    let mut file = File::open(LINE_PATH).unwrap();
    let header = shapefile::header::Header::read_from(&mut file).unwrap();

    assert_eq!(header.shape_type, shapefile::ShapeType::Polyline);
}

fn check_line<T: Read>(mut reader: shapefile::Reader<T>) {
    let shapes = reader.read().unwrap();

    assert_eq!(shapes.len(), 1);
    match &shapes[0] {
        shapefile::record::Shape::Polyline(_poly) => {}
        _ => { assert!(false); }
    }

    if let shapefile::Shape::Polyline(shape) = &shapes[0] {
        assert_eq!(shape.bbox.xmin, 1.0);
        assert_eq!(shape.bbox.ymin, 1.0);
        assert_eq!(shape.bbox.xmax, 5.0);
        assert_eq!(shape.bbox.ymax, 6.0);
        assert_eq!(shape.parts, vec![0, 5]);
        assert_eq!(shape.xs, vec![1.0, 5.0, 5.0, 3.0, 1.0, 3.0, 2.0]);
        assert_eq!(shape.ys, vec![5.0, 5.0, 1.0, 3.0, 1.0, 2.0, 6.0]);
    } else {
        assert!(false, "The shape is not a Polyline");
    }
}

fn check_linem<T: Read>(mut reader: shapefile::Reader<T>) {
    use shapefile::NO_DATA;

    let shapes = reader.read().unwrap();
    assert_eq!(shapes.len(), 1);

    if let shapefile::Shape::PolylineM(shape) = &shapes[0] {
        assert_eq!(shape.bbox.xmin, 1.0);
        assert_eq!(shape.bbox.ymin, 1.0);
        assert_eq!(shape.bbox.xmax, 5.0);
        assert_eq!(shape.bbox.ymax, 6.0);
        assert_eq!(shape.parts, vec![0, 5]);
        assert_eq!(shape.xs, vec![1.0, 5.0, 5.0, 3.0, 1.0, 3.0, 2.0]);
        assert_eq!(shape.ys, vec![5.0, 5.0, 1.0, 3.0, 1.0, 2.0, 6.0]);
        assert_eq!(shape.ms, vec![0.0, NO_DATA, 3.0, NO_DATA, 0.0, NO_DATA, NO_DATA]);
        assert_eq!(shape.m_range, [0.0, 3.0]);
    } else {
        assert!(false, "The shape is not a PolylineM");
    }
}


fn check_linez<T: Read>(mut reader: shapefile::Reader<T>) {
    use shapefile::NO_DATA;
    let shapes = reader.read().unwrap();

    assert_eq!(shapes.len(), 1);

    for shape in shapes {
        if let shapefile::Shape::PolylineZ(shp) = shape {
            assert_eq!(shp.parts, vec![0, 5, 7]);
            assert_eq!(shp.xs, vec![1.0, 5.0, 5.0, 3.0, 1.0, 3.0, 2.0, 3.0, 2.0, 1.0]);
            assert_eq!(shp.ys, vec![5.0, 5.0, 1.0, 3.0, 1.0, 2.0, 6.0, 2.0, 6.0, 9.0]);

            assert_eq!(shp.z_range, [0.0, 22.0]);
            assert_eq!(shp.zs, vec![18.0, 20.0, 22.0, 0.0, 0.0, 0.0, 0.0, 15.0, 13.0, 14.0]);

            assert_eq!(shp.m_range, [0.0, 3.0]);
            assert_eq!(shp.ms, vec![NO_DATA, NO_DATA, NO_DATA, NO_DATA, NO_DATA, NO_DATA, NO_DATA, 0.0, 3.0, 2.0]);
        } else {
            assert!(false, "The shape is not a PolylineZ");
        }
    }
}


fn check_point<T: Read>(mut reader: shapefile::Reader<T>) {
    let shapes = reader.read().unwrap();
    assert_eq!(shapes.len(), 1, "Wrong number of shapes");

    let points = shapefile::record::to_vec_of_point(shapes).unwrap();
    assert_eq!(points.len(), 1, "Wrong number of points");

    let point = &points[0];
    assert_eq!(point.x, 122.0);
    assert_eq!(point.y, 37.0);
}

fn check_pointm<T: Read>(mut reader: shapefile::Reader<T>) {
    let shapes = reader.read().unwrap();
    assert_eq!(shapes.len(), 2, "Wrong number of shapes");

    if let shapefile::Shape::PointM(shp) = &shapes[0] {
        assert_eq!(shp.x, 160477.9000324604);
        assert_eq!(shp.y, 5403959.561417906);
        assert_eq!(shp.m, 0.0);
    } else {
        assert!(false, "The first shape is not a PointZ");
    }

    if let shapefile::Shape::PointM(shp) = &shapes[1] {
        assert_eq!(shp.x, 160467.63787299366);
        assert_eq!(shp.y, 5403971.985031904);
        assert_eq!(shp.m, 0.0);
    } else {
        assert!(false, "The second shape is not a PointZ");
    }
}

fn check_pointz<T: Read>(mut reader: shapefile::Reader<T>) {
    let shapes = reader.read().unwrap();
    assert_eq!(shapes.len(), 2, "Wrong number of shapes");

    if let shapefile::Shape::PointZ(shp) = &shapes[0] {
        assert_eq!(shp.x, 1422464.3681007193);
        assert_eq!(shp.y, 4188962.3364355816);
        assert_eq!(shp.z, 72.40956470558095);
        assert_eq!(shp.m, shapefile::NO_DATA);
    } else {
        assert!(false, "The first shape is not a PointZ");
    }

    if let shapefile::Shape::PointZ(shp) = &shapes[1] {
        assert_eq!(shp.x, 1422459.0908050265);
        assert_eq!(shp.y, 4188942.211755641);
        assert_eq!(shp.z, 72.58286959604922);
        assert_eq!(shp.m, shapefile::NO_DATA);
    } else {
        assert!(false, "The second shape is not a PointZ");
    }
}

fn check_polygon<T: Read>(mut reader: shapefile::Reader<T>) {
    let shapes = reader.read().unwrap();
    assert_eq!(shapes.len(), 1, "Wrong number of shapes");

    if let shapefile::Shape::Polygon(shp) = &shapes[0] {
        assert_eq!(shp.xs, vec![122.0, 117.0, 115.0, 118.0, 113.0, 15.0, 17.0, 22.0, 122.0, 117.0, 115.0]);
        assert_eq!(shp.ys, vec![37.0, 36.0, 32.0, 20.0, 24.0, 2.0, 6.0, 7.0, 37.0, 36.0, 32.0]);
        assert_eq!(shp.parts, vec![0, 5, 8]);
    } else {
        assert!(false, "The second shape is not a Polygon");
    }
}

fn check_polygonz<T: Read>(mut reader: shapefile::Reader<T>) {
    let shapes = reader.read().unwrap();
    assert_eq!(shapes.len(), 1, "Wrong number of shapes");

    //FIXME find a file with less values
    if let shapefile::Shape::PolygonZ(shp) = &shapes[0] {
        assert_eq!(shp.xs, vec![1422692.1644789441, 1422692.1625749937, 1422692.156877633, 1422692.1474302218, 1422692.1343046608, 1422692.1176008438, 1422692.0974458966, 1422692.0739932107, 1422692.047421275, 1422692.017932318, 1422691.9857507686, 1422691.951121548, 1422691.914308205, 1422691.8755909116, 1422691.8352643298, 1422691.7936353693, 1422691.7510208515, 1422691.7077450987, 1422691.6641374656, 1422691.6205298326, 1422691.5772540797, 1422691.534639562, 1422691.4930106015, 1422691.4526840197, 1422691.4139667263, 1422691.3771533833, 1422691.3425241627, 1422691.3103426134, 1422691.2808536564, 1422691.2542817206, 1422691.2308290347, 1422691.2106740875, 1422691.1939702705, 1422691.1808447095, 1422691.1713972983, 1422691.1656999376, 1422691.1637959871, 1422691.1656999376, 1422691.1713972983, 1422691.1808447095, 1422691.1939702705, 1422691.2106740875, 1422691.2308290347, 1422691.2542817206, 1422691.2808536564, 1422691.3103426134, 1422691.3425241627, 1422691.3771533833, 1422691.4139667263, 1422691.4526840197, 1422691.4930106015, 1422691.534639562, 1422691.5772540797, 1422691.6205298326, 1422691.6641374656, 1422691.7077450987, 1422691.7510208515, 1422691.7936353693, 1422691.8352643298, 1422691.8755909116, 1422691.914308205, 1422691.951121548, 1422691.9857507686, 1422692.017932318, 1422692.047421275, 1422692.0739932107, 1422692.0974458966, 1422692.1176008438, 1422692.1343046608, 1422692.1474302218, 1422692.156877633, 1422692.1625749937, 1422692.1644789441]);
        assert_eq!(shp.ys, vec![4188837.794210903, 4188837.75060327, 4188837.7073275167, 4188837.664712999, 4188837.6230840385, 4188837.582757457, 4188837.5440401635, 4188837.5072268206, 4188837.4725976, 4188837.4404160506, 4188837.4109270936, 4188837.384355158, 4188837.360902472, 4188837.3407475245, 4188837.3240437075, 4188837.3109181467, 4188837.3014707356, 4188837.295773375, 4188837.293869424, 4188837.295773375, 4188837.3014707356, 4188837.3109181467, 4188837.3240437075, 4188837.3407475245, 4188837.360902472, 4188837.384355158, 4188837.4109270936, 4188837.4404160506, 4188837.4725976, 4188837.5072268206, 4188837.5440401635, 4188837.582757457, 4188837.6230840385, 4188837.664712999, 4188837.7073275167, 4188837.75060327, 4188837.794210903, 4188837.837818536, 4188837.881094289, 4188837.9237088067, 4188837.9653377673, 4188838.0056643486, 4188838.0443816422, 4188838.081194985, 4188838.115824206, 4188838.148005755, 4188838.177494712, 4188838.2040666477, 4188838.227519334, 4188838.2476742812, 4188838.2643780983, 4188838.277503659, 4188838.28695107, 4188838.292648431, 4188838.2945523816, 4188838.292648431, 4188838.28695107, 4188838.277503659, 4188838.2643780983, 4188838.2476742812, 4188838.227519334, 4188838.2040666477, 4188838.177494712, 4188838.148005755, 4188838.115824206, 4188838.081194985, 4188838.0443816422, 4188838.0056643486, 4188837.9653377673, 4188837.9237088067, 4188837.881094289, 4188837.837818536, 4188837.794210903]);
        assert_eq!(shp.zs, vec![72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523, 72.46632654472523]);
        assert_eq!(shp.m_range, [shapefile::NO_DATA, shapefile::NO_DATA]); // FIXME the file should have these values
        assert_eq!(shp.parts, vec![0]);
    } else {
        assert!(false, "The second shape is not a PolygonZ");
    }
}

macro_rules! read_test {
    ($func:ident, $check_func:ident, $src_file:ident) => {
        #[test]
        fn $func() {
            let reader = shapefile::Reader::from_path($src_file).unwrap();
            $check_func(reader);
        }
    }
}


macro_rules! read_write_read_test {
    ($func:ident, $convert_func:ident, $check_func:ident, $src_file:ident) => {
        #[test]
        fn $func() {
            let mut reader = shapefile::Reader::from_path($src_file).unwrap();
            let shapes = reader.read().unwrap();
            let shapes = $convert_func(shapes).unwrap();

            let v = Vec::<u8>::new();
            let mut cursor = Cursor::new(v);
            let mut writer = shapefile::writer::Writer::new(cursor);
            writer.write_shapes(shapes).unwrap();

            cursor = writer.dest;

            cursor.seek(SeekFrom::Start(0)).unwrap();
            let reader = shapefile::Reader::new(cursor).unwrap();
            $check_func(reader);
        }
    };
}

/* Read tests on Polylines */
read_test!(read_line, check_line, LINE_PATH);
read_test!(read_linem, check_linem, LINEM_PATH);
read_test!(read_linez, check_linez, LINEZ_PATH);

/* Read tests on Points */
read_test!(read_point, check_point, POINT_PATH);
read_test!(read_pointm, check_pointm, POINTM_PATH);
read_test!(read_pointz, check_pointz, POINTZ_PATH);

/* Read tests on Polygon */
read_test!(read_polygon, check_polygon, POLYGON_PATH);
read_test!(read_polygonz, check_polygonz, POLYGONZ_PATH);

/* Read-Write-Read tests on Polylines */
use shapefile::record::{to_vec_of_polyline, to_vec_of_polylinem, to_vec_of_polylinez};
read_write_read_test!(read_write_read_line, to_vec_of_polyline, check_line, LINE_PATH);
read_write_read_test!(read_write_read_linem, to_vec_of_polylinem, check_linem, LINEM_PATH);
read_write_read_test!(read_write_read_linez, to_vec_of_polylinez, check_linez, LINEZ_PATH);

/* Read-Write-Read tests on Points */
use shapefile::record::{to_vec_of_point, to_vec_of_pointm, to_vec_of_pointz};
read_write_read_test!(read_write_read_point, to_vec_of_point, check_point, POINT_PATH);
read_write_read_test!(read_write_read_pointm, to_vec_of_pointm, check_pointm, POINTM_PATH);
read_write_read_test!(read_write_read_pointz, to_vec_of_pointz, check_pointz, POINTZ_PATH);

/* Read-Write-Read tests on Polygons */
use shapefile::record::{to_vec_of_polygon, to_vec_of_polygonz};
read_write_read_test!(read_write_read_polygon, to_vec_of_polygon, check_polygon, POLYGON_PATH);
read_write_read_test!(read_write_read_polygonz, to_vec_of_polygonz, check_polygonz, POLYGONZ_PATH);
