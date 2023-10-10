use std::io::BufRead;

fn main() -> Result<()> {
    let args = std::env::args();
    let filepath = args.skip(1).next();
    if filepath == None {
        println!("Usage:\n\tprogramm <input_filepath>\n");
        std::process::exit(1);
    }

    let filepath: std::path::PathBuf = filepath.unwrap().into();
    let file: std::fs::File = std::fs::File::open(filepath)?;
    let reader = std::io::BufReader::new(file);

    let wrapping_paper: u32 = reader
        .lines()
        .flat_map(|l| l.map(|l| PresentBox::try_from(l.as_str())))
        .map(|b| b.map(|b| b.wrapping_paper_needed()))
        .try_fold(0, |acc, x| -> Result<u32> { Ok(acc + x?) })?;
    println!("Wrapping paper needed: {}", wrapping_paper);

    Ok(())
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    InvalidInput,
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

struct PresentBox {
    height: u8,
    width: u8,
    length: u8,
}

impl PresentBox {
    fn get_surface_area(&self) -> u32 {
        2 * self.height as u32 * self.width as u32
            + 2 * self.width as u32 * self.length as u32
            + 2 * self.length as u32 * self.height as u32
    }

    fn get_smallest_side_area(&self) -> u32 {
        let mut sides = [self.height, self.width, self.length];
        sides.sort();
        sides[0] as u32 * sides[1] as u32
    }

    fn wrapping_paper_needed(&self) -> u32 {
        self.get_surface_area() + self.get_smallest_side_area()
    }
}

impl TryFrom<&str> for PresentBox {
    type Error = Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let mut parts = value.split('x');
        let (height, width, length) = (
            parts.next().ok_or(Error::InvalidInput)?,
            parts.next().ok_or(Error::InvalidInput)?,
            parts.next().ok_or(Error::InvalidInput)?,
        );
        let (height, width, length) = (
            height.parse().map_err(|_| Error::InvalidInput)?,
            width.parse().map_err(|_| Error::InvalidInput)?,
            length.parse().map_err(|_| Error::InvalidInput)?,
        );
        Ok(PresentBox {
            height,
            width,
            length,
        })
    }
}
