use std::collections::HashMap;
use std::fs::{FileType, Permissions};
use std::os::unix::fs::{FileTypeExt, PermissionsExt};
use std::path::Path;
use crate::lib::util::walker::{Walker, WakerEntry};
use ansi_term::{Colour, Style, ANSIString};

type ColorChars = [char; 22];

/// Make an `ANSIString` for specify str with foreground char and background char
///
/// # Arguments
///
/// * `s` - displaying string
/// * `fg` - foreground char
/// * `bg` - background char
///
/// # Return
/// * ANSIString
fn to_ansi_string(s: &str, fg: char, bg: char) -> ANSIString {
    let mut style = Style::default();
    // ref: https://gist.github.com/thomd/7667642
    let map_to_colour = |ch: char| {
        match ch.to_ascii_lowercase() {
            'a' => { Some(Colour::Black) },
            'b' => { Some(Colour::Red) },
            'c' => { Some(Colour::Green) },
            'd' => { Some(Colour::Yellow) },
            'e' => { Some(Colour::Blue) },
            'f' => { Some(Colour::Purple) },
            'g' => { Some(Colour::Cyan) },
            'h' => { Some(Colour::White) },
            _ => { None }
        }
    };
    style.foreground = map_to_colour(fg);
    style.background = map_to_colour(bg);
    style.is_bold = fg >= 'A' && fg <= 'H';
    style.paint(s)
}

fn color_index(file_type: FileType, permissions: Permissions) -> Option<usize> {
    // 参考：https://gist.github.com/thomd/7667642
    if file_type.is_symlink() { return Some(2) }
    if file_type.is_fifo() { return Some(3) }
    if file_type.is_socket() { return Some(4) }
    if file_type.is_block_device() { return Some(6) }
    if file_type.is_char_device() { return Some(7) }
    if file_type.is_file() {
        return if permissions.mode() & 0o111 != 0 {
            Some(5)
        } else {
            None
        }
        // TODO: 需要更精细化的处理
        // executable -> 5
        // executable with setuid bit set -> 8
        // executable with setgid bit set -> 9
    }
    if file_type.is_dir() {
        return Some(1)
        // TODO: 需要更精细化的处理
        // directory writable to others, with sticky bit -> 10
        // directory writable to others, without sticky bit -> 11
    }
    None
}

fn get_color_chars(file_type: FileType, permissions: Permissions, map: &ColorChars) -> Option<(char, char)> {
    let index = color_index(file_type, permissions)?;
    let fg_i = (index - 1) * 2;
    let bg_i = fg_i + 1;
    Some((map[fg_i].clone(), map[bg_i].clone()))
}

pub fn walk<P: AsRef<Path>>(dir: P) {
    let mut walker = Walker::new(dir);

    let color_chars: Option<ColorChars> =
        if let Some(s) = std::env::var("LSCOLORS").ok() {
            if s.chars().count() == 22 {
                let mut ret = ColorChars::default();
                let mut from_iter = s.chars().into_iter();
                let mut i = 0;
                while let Some(v) = from_iter.next() {
                    ret[i] = v;
                    i += 1;
                }
                Some(ret)
            } else {
                None
            }
        } else {
            None
        };
    let display_file_name = |entry: &WakerEntry| -> String {
        let file_name= entry.inner.file_name();
        let file_name= file_name.to_str().unwrap_or("NULL");
        if color_chars.is_none() { return file_name.to_string() }
        let file_type = entry.inner.file_type().ok();
        let permissions = entry.inner.metadata().ok().map(|m| m.permissions());
        if file_type.is_none() { return file_name.to_string() }
        if permissions.is_none() { return file_name.to_string() }
        if let Some(cs) = get_color_chars(file_type.unwrap(), permissions.unwrap(), &color_chars.unwrap()) {
            to_ansi_string(file_name, cs.0, cs.1).to_string()
        } else {
            file_name.to_string()
        }
    };
    type DepthSiblings = HashMap<usize, bool>;
    let depth_siblings: DepthSiblings = HashMap::new();
    let print = |entry: WakerEntry| {
        let mut prefix: Vec<char> = Vec::new();
        let (nbsp, space) = (char::from(0xa0), ' ');
        for i in 1..entry.depth {
            if *depth_siblings.get(&i).unwrap_or(&false) {
                prefix.extend_from_slice(&['│', nbsp, nbsp, space]);
            } else {
                prefix.extend_from_slice(&[space; 4]);
            }
        }
        let c = if entry.has_next_sibling { '├' } else { '└' };
        prefix.extend_from_slice(&[c, '─', '─', space]);
        let prefix= prefix.into_iter().collect::<String>();
        let prefix = prefix.as_str();
        let line_color = Colour::RGB(94, 94, 94); // light gray
        println!(
            "{}{}",
            Style::from(line_color).paint(prefix),
            display_file_name(&entry)
        );

        // TODO: 此处想直接 insert，失败，就采用 unsafe 的方式绕过
        // depth_siblings.insert(entry.depth, entry.has_next_sibling);
        let mu = &depth_siblings as *const DepthSiblings as *mut DepthSiblings;
        unsafe { (*mu).insert(entry.depth, entry.has_next_sibling); }
    };
    walker.max_depth(3).start(&print).unwrap();
}
