#[derive(Debug, Clone)]
pub struct Canvas<T>
{
    width: usize,
    height: usize,
    elems: Vec<T>,
}

impl<T> Canvas<T>
{
    pub fn empty() -> Canvas<T>
    {
        Canvas {
            width: 0,
            height: 0,
            elems: vec![],
        }
    }

    pub fn new<F>(width: usize, height: usize, mut generator: F) -> Canvas<T>
        where F: FnMut((usize, usize)) -> T
    {
        let mut elems = Vec::with_capacity(width * height);

        for y in 0..height
        {
            for x in 0..width
            {
                elems.push(generator((x, y)));
            }
        }

        Canvas {
            width,
            height,
            elems,
        }
    }

    pub fn width(&self) -> usize
    {
        self.width
    }

    pub fn height(&self) -> usize
    {
        self.height
    }

    pub fn map<F, O>(self, f: F) -> Canvas<O>
    where F: Fn(T) -> O
    {
        Canvas {
            width: self.width,
            height: self.height,
            elems: self.elems.into_iter().map(f).collect(),
        }
    }

    pub fn for_each<F>(&self, mut f: F)
    where F: FnMut((usize, usize), &T)
    {
        for y in 0..self.height
        {
            for x in 0..self.width
            {
                f((x, y), &self.elems[x + y * self.width]);
            }
        }
    }
}