use std::{
    collections::VecDeque,
    io::{BufRead, Write},
};

pub fn write_lines<W, R>(out: &mut W, reader: R) -> std::io::Result<()>
where
    W: Write,
    R: BufRead,
{
    for line in reader.lines() {
        writeln!(out, "{}", line?)?;
    }
    Ok(())
}

pub fn write_lines_partial<W, R>(
    out: &mut W,
    reader: R,
    head: Option<usize>,
    tail: Option<usize>,
) -> std::io::Result<()>
where
    W: Write,
    R: BufRead,
{
    if head.is_none() && tail.is_none() {
        write_lines(out, reader)?;
    } else {
        let mut total_lines = 0;
        let head_count = head.unwrap_or(0);
        let mut head = Vec::with_capacity(head_count);
        let tail_count = tail.unwrap_or(0);
        let mut tail = VecDeque::with_capacity(tail_count);

        for line in reader.lines() {
            let line = line?;
            total_lines += 1;
            if head.len() < head_count {
                head.push(line.clone());
            }
            while tail.len() >= tail_count {
                tail.pop_front();
            }
            tail.push_back(line);
        }

        let overlap = (head.len() + tail.len()).saturating_sub(total_lines);
        for _ in 0..overlap {
            tail.pop_front();
        }

        let omitted = total_lines.saturating_sub(head.len() + tail.len());
        for line in head {
            writeln!(out, "{}", line)?;
        }
        if omitted > 0 {
            writeln!(out, "[... {} lines omitted]", omitted)?;
        }
        for line in tail {
            writeln!(out, "{}", line)?;
        }
    }

    Ok(())
}
