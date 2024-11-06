#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct Rectangle {
    a: Point, // верхняя левая точка
    b: Point, // нижняя правая точка
}

impl Rectangle {
    // Метод для вычисления площади прямоугольника
    fn area(&self) -> i32 {
        let width = (self.b.x - self.a.x).abs();
        let height = (self.a.y - self.b.y).abs();
        width * height
    }
}

// Функция для нахождения пересечения двух прямоугольников
fn intersection(r1: &Rectangle, r2: &Rectangle) -> Option<Rectangle> {
    let left = r1.a.x.max(r2.a.x);
    let right = r1.b.x.min(r2.b.x);
    let top = r1.a.y.min(r2.a.y);
    let bottom = r1.b.y.max(r2.b.y);

    if left < right && top > bottom {
        Some(Rectangle {
            a: Point { x: left, y: top },
            b: Point { x: right, y: bottom },
        })
    } else {
        None
    }
}

// Функция для вычисления фактически занятой площади с учетом перекрытий
fn area_occupied(rectangles: &Vec<Rectangle>) -> i32 {
    let mut total_area = 0;
    let mut overlap_area = 0;

    // Считаем площадь всех прямоугольников
    for rect in rectangles {
        total_area += rect.area();
    }

    // Считаем площадь перекрытий
    for i in 0..rectangles.len() {
        for j in (i + 1)..rectangles.len() {
            if let Some(overlap) = intersection(&rectangles[i], &rectangles[j]) {
                overlap_area += overlap.area();
            }
        }
    }

    total_area - overlap_area
}

// Тестовые данные
fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

// Тестовая функция для проверки вычисления занятой площади
fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
    println!("Test passed: occupied area = {}", occupied);
}

fn main() {
    area_occupied_test();
}
