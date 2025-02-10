use std::{
    fs::File,
    io::{stdin, stdout, BufRead, BufReader, Read, Write},
    path::Path,
};

use serde::{de::DeserializeOwned, Serialize};

fn open<P>(path: P) -> anyhow::Result<Box<dyn Read>>
where
    P: AsRef<Path>,
{
    if is_stdin(path.as_ref()) {
        Ok(Box::new(stdin()))
    } else {
        Ok(Box::new(File::open(path.as_ref())?))
    }
}

pub fn read_json<D, P>(path: P) -> anyhow::Result<D>
where
    D: DeserializeOwned,
    P: AsRef<Path>,
{
    Ok(serde_json::from_reader(open(path)?)?)
}

pub fn read_string<P>(path: P) -> anyhow::Result<String>
where
    P: AsRef<Path>,
{
    let mut out = String::new();
    read_to_string(&mut out, path)?;
    Ok(out)
}

pub fn read_to_string<P>(out: &mut String, path: P) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    open(path)?.read_to_string(out)?;
    Ok(())
}

pub fn print_json<S>(value: &S) -> anyhow::Result<()>
where
    S: Serialize,
{
    serde_json::to_writer(stdout(), value)?;
    Ok(())
}

pub fn cat<W, P>(out: &mut W, path: P) -> anyhow::Result<()>
where
    P: AsRef<Path>,
    W: Write,
{
    cat_reader(out, BufReader::new(open(path)?))?;
    Ok(())
}

pub fn cat_reader<W, R>(out: &mut W, reader: R) -> anyhow::Result<()>
where
    W: Write,
    R: BufRead,
{
    for line in reader.lines() {
        writeln!(out, "{}", line?)?;
    }
    Ok(())
}

fn is_stdin<P>(path: P) -> bool
where
    P: AsRef<Path>,
{
    path.as_ref() == Path::new("-")
}
