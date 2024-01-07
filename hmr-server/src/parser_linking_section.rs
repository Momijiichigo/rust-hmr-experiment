use nom::{
    bytes::complete::{is_not, tag, take, take_until, take_while},
    character::complete::char,
    character::is_digit,
    combinator::map_res,
    error::{Error, ContextError, ErrorKind, ParseError},
    sequence::{delimited, tuple},
    Err, IResult,
};

/// parses the linking section of a wasm binary.
/// returns a list of symbol info
pub fn take_linking_section(input: &[u8]) -> IResult<&[u8], Vec<SymbolInfo>> {
    // # version
    let (input, version) = take(1usize)(input)?;

    if version != [2] {
        return error_with_context(input, "version is not 2");
    }

    // # subsections
    // ## type
    let (input, subsection_type) = take(1usize)(input)?;

    if subsection_type != [8] {
        return error_with_context(input, "subsection type is not 8");
    }

    // ## payload length
    let (input, _payload_length) = take_var_uint_32(input)?;

    // # WASM_SYMBOL_TABLE
    // ## count of syminfo
    let (input, count) = take_var_uint_32(input)?;

    let mut syminfo_list = Vec::with_capacity(count as usize);
    let mut input = input;
    for _ in 0..count {
        let (i, syminfo) = take_syminfo(input)?;
        input = i;
        syminfo_list.push(syminfo);
    }

    Ok((input, syminfo_list))
}

fn take_var_uint_32(input: &[u8]) -> IResult<&[u8], u32> {
    let mut res = 0;
    let mut shift = 0;

    let mut input = input;
    loop {
        if shift > 31 {
            return Err(nom::Err::Failure(Error::new(input, ErrorKind::Fail)));
        }

        let (i, byte) = take(1usize)(input)?;
        input = i;
        let byte = byte[0] as u32;

        res |= (byte & 0x7f)
            .checked_shl(shift)
            .ok_or(Err::Error(Error::new(input, ErrorKind::Fail)))?;
        shift += 7;
        if (byte >> 7) == 0 {
            if shift >= 32 && (byte as u8).leading_zeros() < 4 {
                return Err(Err::Error(Error::new(input, ErrorKind::Fail)));
            }
            break;
        }
    }

    Ok((input, res))
}

#[derive(Debug)]
pub enum SymbolInfo {
    Function(u32, Option<String>, bool),
    Data(Option<u32>, String),
    Global(u32, Option<String>, bool),
    Section(u32),
    Event(u32, Option<String>, bool),
    Table(u32, Option<String>, bool),
}
fn take_syminfo(input: &[u8]) -> IResult<&[u8], SymbolInfo> {
    let (input, kind) = take(1usize)(input)?;
    let (input, flags) = take_var_uint_32(input)?;
    let mut remaining = input;

    let kind = match kind[0] {
        0x00 => {
            let (input, (idx, name, is_imported)) = take_general_symbol(remaining, flags)?;
            remaining = input;
            SymbolInfo::Function(idx, name, is_imported)
        }
        0x01 => {
            let (input, (idx, name)) = take_data_symbol(remaining, flags)?;
            remaining = input;
            SymbolInfo::Data(idx, name)
        },
        0x02 => {
            let (input, (idx, name, is_imported)) = take_general_symbol(remaining, flags)?;
            remaining = input;
            SymbolInfo::Global(idx, name, is_imported)
        },
        0x03 => {
            let (input, idx) = take_var_uint_32(remaining)?;
            remaining = input;
            SymbolInfo::Section(idx)
        },
        0x04 => {
            let (input, (idx, name, is_imported)) = take_general_symbol(remaining, flags)?;
            remaining = input;
            SymbolInfo::Event(idx, name, is_imported)
        },
        0x05 => {
            let (input, (idx, name, is_imported)) = take_general_symbol(remaining, flags)?;
            remaining = input;
            SymbolInfo::Table(idx, name, is_imported)
        },
        _ => return error_with_context(input, "invalid symbol kind"),
    };


    Ok((remaining, kind))
}

fn take_general_symbol(input: &[u8], flags: u32) -> IResult<&[u8], (u32, Option<String>, bool)> {
    let (input, idx) = take_var_uint_32(input)?;
    let mut remaining = input;

    let mut name = None;
    let is_imported = (flags & 0x10) != 0;
    // if the symbol does not reference import
    if !is_imported || (flags & 0x40) != 0 {
        let (input, name_len) = take_var_uint_32(input)?;
        let (input, name_bytes) = take(name_len as usize)(input)?;
        remaining = input;
        name = Some(
            String::from_utf8(name_bytes.to_vec())
                .map_err(|_| Err::Error(Error::new(input, ErrorKind::Fail)))?,
        );
    }

    Ok((remaining, (idx, name, is_imported)))
}

fn take_data_symbol(input: &[u8], flags: u32) -> IResult<&[u8], (Option<u32>, String)> {
    let (input, name_len) = take_var_uint_32(input)?;
    let (input, name_bytes) = take(name_len as usize)(input)?;
    let name = String::from_utf8(name_bytes.to_vec())
        .map_err(|_| Err::Error(Error::new(input, ErrorKind::Fail)))?;

    let mut idx = None;
    let mut remaining = input;

    // if symbol is defined
    if (flags & 0x10) == 0 {
        let (input, idx_) = take_var_uint_32(input)?;
        idx = Some(idx_);
        let (input, _offset) = take_var_uint_32(input)?;
        let (input, _size) = take_var_uint_32(input)?;
        remaining = input;
    }


    Ok((remaining, (idx, name)))
}
fn error_with_context<I: Copy,O>(input: I, msg: &'static str) -> IResult<I,O,Error<I>> {
    Err(Err::Error(ContextError::add_context(input, msg, Error::new(input, ErrorKind::Fail))))
}
