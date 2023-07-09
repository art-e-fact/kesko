// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code)]
pub mod kesko {

  use core::mem;
  use core::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};
#[allow(unused_imports, dead_code)]
pub mod urdf {

  use core::mem;
  use core::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};

// struct Vec3, aligned to 4
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct Vec3(pub [u8; 12]);
impl Default for Vec3 { 
  fn default() -> Self { 
    Self([0; 12])
  }
}
impl core::fmt::Debug for Vec3 {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    f.debug_struct("Vec3")
      .field("x", &self.x())
      .field("y", &self.y())
      .field("z", &self.z())
      .finish()
  }
}

impl flatbuffers::SimpleToVerifyInSlice for Vec3 {}
impl<'a> flatbuffers::Follow<'a> for Vec3 {
  type Inner = &'a Vec3;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a Vec3>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a Vec3 {
  type Inner = &'a Vec3;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<Vec3>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for Vec3 {
    type Output = Vec3;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        let src = ::core::slice::from_raw_parts(self as *const Vec3 as *const u8, Self::size());
        dst.copy_from_slice(src);
    }
}

impl<'a> flatbuffers::Verifiable for Vec3 {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.in_buffer::<Self>(pos)
  }
}

impl<'a> Vec3 {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    x: f32,
    y: f32,
    z: f32,
  ) -> Self {
    let mut s = Self([0; 12]);
    s.set_x(x);
    s.set_y(y);
    s.set_z(z);
    s
  }

  pub fn x(&self) -> f32 {
    let mut mem = core::mem::MaybeUninit::<<f32 as EndianScalar>::Scalar>::uninit();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    EndianScalar::from_little_endian(unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[0..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<<f32 as EndianScalar>::Scalar>(),
      );
      mem.assume_init()
    })
  }

  pub fn set_x(&mut self, x: f32) {
    let x_le = x.to_little_endian();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const _ as *const u8,
        self.0[0..].as_mut_ptr(),
        core::mem::size_of::<<f32 as EndianScalar>::Scalar>(),
      );
    }
  }

  pub fn y(&self) -> f32 {
    let mut mem = core::mem::MaybeUninit::<<f32 as EndianScalar>::Scalar>::uninit();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    EndianScalar::from_little_endian(unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[4..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<<f32 as EndianScalar>::Scalar>(),
      );
      mem.assume_init()
    })
  }

  pub fn set_y(&mut self, x: f32) {
    let x_le = x.to_little_endian();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const _ as *const u8,
        self.0[4..].as_mut_ptr(),
        core::mem::size_of::<<f32 as EndianScalar>::Scalar>(),
      );
    }
  }

  pub fn z(&self) -> f32 {
    let mut mem = core::mem::MaybeUninit::<<f32 as EndianScalar>::Scalar>::uninit();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    EndianScalar::from_little_endian(unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[8..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<<f32 as EndianScalar>::Scalar>(),
      );
      mem.assume_init()
    })
  }

  pub fn set_z(&mut self, x: f32) {
    let x_le = x.to_little_endian();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const _ as *const u8,
        self.0[8..].as_mut_ptr(),
        core::mem::size_of::<<f32 as EndianScalar>::Scalar>(),
      );
    }
  }

}

pub enum PackageMapOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct PackageMap<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for PackageMap<'a> {
  type Inner = PackageMap<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> PackageMap<'a> {
  pub const VT_PACKAGE_NAME: flatbuffers::VOffsetT = 4;
  pub const VT_PACKAGE_PATH: flatbuffers::VOffsetT = 6;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    PackageMap { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args PackageMapArgs<'args>
  ) -> flatbuffers::WIPOffset<PackageMap<'bldr>> {
    let mut builder = PackageMapBuilder::new(_fbb);
    if let Some(x) = args.package_path { builder.add_package_path(x); }
    if let Some(x) = args.package_name { builder.add_package_name(x); }
    builder.finish()
  }


  #[inline]
  pub fn package_name(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(PackageMap::VT_PACKAGE_NAME, None)}
  }
  #[inline]
  pub fn package_path(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(PackageMap::VT_PACKAGE_PATH, None)}
  }
}

impl flatbuffers::Verifiable for PackageMap<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("package_name", Self::VT_PACKAGE_NAME, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("package_path", Self::VT_PACKAGE_PATH, false)?
     .finish();
    Ok(())
  }
}
pub struct PackageMapArgs<'a> {
    pub package_name: Option<flatbuffers::WIPOffset<&'a str>>,
    pub package_path: Option<flatbuffers::WIPOffset<&'a str>>,
}
impl<'a> Default for PackageMapArgs<'a> {
  #[inline]
  fn default() -> Self {
    PackageMapArgs {
      package_name: None,
      package_path: None,
    }
  }
}

pub struct PackageMapBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> PackageMapBuilder<'a, 'b> {
  #[inline]
  pub fn add_package_name(&mut self, package_name: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(PackageMap::VT_PACKAGE_NAME, package_name);
  }
  #[inline]
  pub fn add_package_path(&mut self, package_path: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(PackageMap::VT_PACKAGE_PATH, package_path);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> PackageMapBuilder<'a, 'b> {
    let start = _fbb.start_table();
    PackageMapBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<PackageMap<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for PackageMap<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("PackageMap");
      ds.field("package_name", &self.package_name());
      ds.field("package_path", &self.package_path());
      ds.finish()
  }
}
pub enum SpawnUrdfOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct SpawnUrdf<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for SpawnUrdf<'a> {
  type Inner = SpawnUrdf<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> SpawnUrdf<'a> {
  pub const VT_POSITION: flatbuffers::VOffsetT = 4;
  pub const VT_URDF_PATH: flatbuffers::VOffsetT = 6;
  pub const VT_PACKAGE_MAPPINGS: flatbuffers::VOffsetT = 8;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    SpawnUrdf { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args SpawnUrdfArgs<'args>
  ) -> flatbuffers::WIPOffset<SpawnUrdf<'bldr>> {
    let mut builder = SpawnUrdfBuilder::new(_fbb);
    if let Some(x) = args.package_mappings { builder.add_package_mappings(x); }
    if let Some(x) = args.urdf_path { builder.add_urdf_path(x); }
    if let Some(x) = args.position { builder.add_position(x); }
    builder.finish()
  }


  #[inline]
  pub fn position(&self) -> Option<&'a Vec3> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<Vec3>(SpawnUrdf::VT_POSITION, None)}
  }
  #[inline]
  pub fn urdf_path(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(SpawnUrdf::VT_URDF_PATH, None)}
  }
  #[inline]
  pub fn package_mappings(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<PackageMap<'a>>>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<PackageMap>>>>(SpawnUrdf::VT_PACKAGE_MAPPINGS, None)}
  }
}

impl flatbuffers::Verifiable for SpawnUrdf<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<Vec3>("position", Self::VT_POSITION, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("urdf_path", Self::VT_URDF_PATH, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<PackageMap>>>>("package_mappings", Self::VT_PACKAGE_MAPPINGS, false)?
     .finish();
    Ok(())
  }
}
pub struct SpawnUrdfArgs<'a> {
    pub position: Option<&'a Vec3>,
    pub urdf_path: Option<flatbuffers::WIPOffset<&'a str>>,
    pub package_mappings: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<PackageMap<'a>>>>>,
}
impl<'a> Default for SpawnUrdfArgs<'a> {
  #[inline]
  fn default() -> Self {
    SpawnUrdfArgs {
      position: None,
      urdf_path: None,
      package_mappings: None,
    }
  }
}

pub struct SpawnUrdfBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> SpawnUrdfBuilder<'a, 'b> {
  #[inline]
  pub fn add_position(&mut self, position: &Vec3) {
    self.fbb_.push_slot_always::<&Vec3>(SpawnUrdf::VT_POSITION, position);
  }
  #[inline]
  pub fn add_urdf_path(&mut self, urdf_path: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(SpawnUrdf::VT_URDF_PATH, urdf_path);
  }
  #[inline]
  pub fn add_package_mappings(&mut self, package_mappings: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<PackageMap<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(SpawnUrdf::VT_PACKAGE_MAPPINGS, package_mappings);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> SpawnUrdfBuilder<'a, 'b> {
    let start = _fbb.start_table();
    SpawnUrdfBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<SpawnUrdf<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for SpawnUrdf<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("SpawnUrdf");
      ds.field("position", &self.position());
      ds.field("urdf_path", &self.urdf_path());
      ds.field("package_mappings", &self.package_mappings());
      ds.finish()
  }
}
#[inline]
/// Verifies that a buffer of bytes contains a `SpawnUrdf`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_spawn_urdf_unchecked`.
pub fn root_as_spawn_urdf(buf: &[u8]) -> Result<SpawnUrdf, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root::<SpawnUrdf>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `SpawnUrdf` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_spawn_urdf_unchecked`.
pub fn size_prefixed_root_as_spawn_urdf(buf: &[u8]) -> Result<SpawnUrdf, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root::<SpawnUrdf>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `SpawnUrdf` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_spawn_urdf_unchecked`.
pub fn root_as_spawn_urdf_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<SpawnUrdf<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root_with_opts::<SpawnUrdf<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `SpawnUrdf` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_spawn_urdf_unchecked`.
pub fn size_prefixed_root_as_spawn_urdf_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<SpawnUrdf<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root_with_opts::<SpawnUrdf<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a SpawnUrdf and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `SpawnUrdf`.
pub unsafe fn root_as_spawn_urdf_unchecked(buf: &[u8]) -> SpawnUrdf {
  flatbuffers::root_unchecked::<SpawnUrdf>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed SpawnUrdf and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `SpawnUrdf`.
pub unsafe fn size_prefixed_root_as_spawn_urdf_unchecked(buf: &[u8]) -> SpawnUrdf {
  flatbuffers::size_prefixed_root_unchecked::<SpawnUrdf>(buf)
}
#[inline]
pub fn finish_spawn_urdf_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<SpawnUrdf<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_spawn_urdf_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<SpawnUrdf<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
}  // pub mod urdf
}  // pub mod kesko
