#[derive(Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` methods go in here
impl Point {
    // This is a static method
    // Static methods don't need to be called by an instance
    // These methods are generally used as constructors
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another static method, taking two arguments:
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
    is_square: bool,
}

impl Rectangle {
    fn new(p1: Point, p2: Point) -> Rectangle {
        Rectangle {
            p1,
            p2,
            is_square: (p1.x - p2.x).abs() == (p1.y - p2.y).abs(),
        }
    }

    // This is an instance method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }

    fn scale(&mut self, x_factor: f64, y_factor: f64) {
        let x_diff = (self.p2.x - self.p1.x) * x_factor;
        let y_diff = (self.p2.y - self.p1.y) * y_factor;
        self.p2.x = self.p1.x + x_diff;
        self.p2.y = self.p1.y + y_diff;
    }

    fn mutate<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Point, &mut Point)
    {
        f(&mut self.p1, &mut self.p2)
    }
}

impl Drop for Rectangle {
    fn drop(&mut self) {
        assert!(!self.is_square ||
                (self.p2.x - self.p1.x).abs() == (self.p2.y - self.p1.y).abs());
    }
}

// `Pair` owns resources: two heap allocated integers
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    fn destroy(self) {
        // Destructure `self`
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
    }
}

fn main() {
    let mut rectangle = Rectangle::new(
        Point::origin(),
        Point::new(3.0, 4.0),
    );

    // Instance methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle::new(
        Point::origin(),
        Point::new(1.0, 1.0),
    );

    rectangle.translate(1.0, 0.0);

    // Okay! Mutable objects can call mutable methods
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    square.scale(1.0, 0.9);

    square.translate(-1.0, 3.5);

    rectangle.scale(2.0, 3.5);

    let mut rectangle2 = Rectangle::new(
        Point::origin(),
        Point::new(3.0, 4.0),
    );

    rectangle2.mutate(|p1, _p2| p1.x = 5.3);
    rectangle2.mutate(|p1, p2| {
        p1.x = 90.0;
        p2.y = -3.2;
    });

    pair.destroy();
}
