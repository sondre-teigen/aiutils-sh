use std::{
    fs::File,
    io::{stdin, stdout, BufRead, BufReader, Lines, Read, Result, Write},
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

// REVIEW

pub fn cat<W, P>(out: &mut W, path: P) -> Result<()>
where
    P: AsRef<Path>,
    W: Write,
{
    if is_stdin(path.as_ref()) {
        cat_stdin(out)?;
    } else {
        cat_file(out, path.as_ref())?;
    }
    Ok(())
}

fn is_stdin<P>(path: P) -> bool
where
    P: AsRef<Path>,
{
    path.as_ref() == Path::new("-")
}

fn cat_stdin<W>(out: &mut W) -> Result<()>
where
    W: Write,
{
    cat_lines(out, stdin().lines())?;

    Ok(())
}

fn cat_file<W, P>(out: &mut W, path: P) -> Result<()>
where
    P: AsRef<Path>,
    W: Write,
{
    cat_lines(out, BufReader::new(File::open(path.as_ref())?).lines())?;
    Ok(())
}

fn cat_lines<W, R>(out: &mut W, lines: Lines<R>) -> Result<()>
where
    W: Write,
    R: BufRead,
{
    for line in lines {
        writeln!(out, "{}", line?)?;
    }
    Ok(())
}
