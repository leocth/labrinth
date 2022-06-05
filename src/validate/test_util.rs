use bytes::Bytes;
use std::io::Cursor;
use zip::{result::ZipResult, write::FileOptions, ZipArchive, ZipWriter};

/// Makes a dummy ZIP file with named blank files.
/// Used for unit tests.
#[cfg(test)]
pub fn make_dummy_zip(
    filenames: &[&str],
) -> ZipResult<ZipArchive<Cursor<bytes::Bytes>>> {
    let mut zip = ZipWriter::new(Cursor::new(vec![]));
    let options = FileOptions::default();
    for &file in filenames {
        zip.start_file(file, options)?;
    }
    let data = zip.finish()?.into_inner();

    let data = Bytes::from(data);
    ZipArchive::new(Cursor::new(data))
}
