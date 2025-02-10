use std::{
    fs::File,
    io::{stdin, BufReader, Read, Write},
    path::Path,
};

use serde::de::DeserializeOwned;

pub fn open<P>(path: P) -> anyhow::Result<Box<dyn Read>>
where
    P: AsRef<Path>,
{
    if is_stdin(path.as_ref()) {
        Ok(Box::new(stdin()))
    } else {
        Ok(Box::new(File::open(path.as_ref())?))
    }
}

pub fn open_buffered<P>(path: P) -> anyhow::Result<BufReader<Box<dyn Read>>>
where
    P: AsRef<Path>,
{
    Ok(BufReader::new(open(path.as_ref())?))
}

pub fn read_json<D, P>(path: P) -> anyhow::Result<D>
where
    D: DeserializeOwned,
    P: AsRef<Path>,
{
    Ok(serde_json::from_reader(open_buffered(path)?)?)
}

pub fn read_string<P>(path: P) -> anyhow::Result<String>
where
    P: AsRef<Path>,
{
    let mut out = String::new();
    open(path)?.read_to_string(&mut out)?;
    Ok(out)
}

pub fn cat<W, P>(out: &mut W, path: P) -> anyhow::Result<()>
where
    P: AsRef<Path>,
    W: Write,
{
    crate::io::write_lines(out, open_buffered(path)?)?;
    Ok(())
}

fn is_stdin<P>(path: P) -> bool
where
    P: AsRef<Path>,
{
    path.as_ref() == Path::new("-")
}
