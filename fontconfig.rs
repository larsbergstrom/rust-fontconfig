// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[allow(non_uppercase_statics)];

use std::libc::*;

pub type FcChar8 = c_uchar;
pub type FcChar16 = c_ushort;
pub type FcChar32 = c_uint;
pub type FcBool = c_int;

pub type enum__FcType = c_uint;
pub static FcTypeVoid: u32 = 0_u32;
pub static FcTypeInteger: u32 = 1_u32;
pub static FcTypeDouble: u32 = 2_u32;
pub static FcTypeString: u32 = 3_u32;
pub static FcTypeBool: u32 = 4_u32;
pub static FcTypeMatrix: u32 = 5_u32;
pub static FcTypeCharSet: u32 = 6_u32;
pub static FcTypeFTFace: u32 = 7_u32;
pub static FcTypeLangSet: u32 = 8_u32;

pub type FcType = enum__FcType;

pub static FC_WEIGHT_THIN: c_int = 0;
pub static FC_WEIGHT_EXTRALIGHT: c_int = 40;
pub static FC_WEIGHT_ULTRALIGHT: c_int = FC_WEIGHT_EXTRALIGHT;
pub static FC_WEIGHT_LIGHT: c_int = 50;
pub static FC_WEIGHT_BOOK: c_int = 75;
pub static FC_WEIGHT_REGULAR: c_int = 80;
pub static FC_WEIGHT_NORMAL: c_int = FC_WEIGHT_REGULAR;
pub static FC_WEIGHT_MEDIUM: c_int = 100;
pub static FC_WEIGHT_DEMIBOLD: c_int = 180;
pub static FC_WEIGHT_SEMIBOLD: c_int = FC_WEIGHT_DEMIBOLD;
pub static FC_WEIGHT_BOLD: c_int = 200;
pub static FC_WEIGHT_EXTRABOLD: c_int = 205;
pub static FC_WEIGHT_ULTRABOLD: c_int = FC_WEIGHT_EXTRABOLD;
pub static FC_WEIGHT_BLACK: c_int = 210;
pub static FC_WEIGHT_HEAVY: c_int = FC_WEIGHT_BLACK;
pub static FC_WEIGHT_EXTRABLACK: c_int = 215;
pub static FC_WEIGHT_ULTRABLACK: c_int = FC_WEIGHT_EXTRABLACK;

pub static FC_SLANT_ROMAN: c_int = 0;
pub static FC_SLANT_ITALIC: c_int = 100;
pub static FC_SLANT_OBLIQUE: c_int = 110;

pub struct struct__FcMatrix {
    xx: c_double,
    xy: c_double,
    yx: c_double,
    yy: c_double,
}

pub type FcMatrix = struct__FcMatrix;

pub type struct__FcCharSet = c_void;

pub type FcCharSet = struct__FcCharSet;

pub struct struct__FcObjectType {
    object: *c_char,
    _type: FcType,
}

pub type FcObjectType = struct__FcObjectType;

pub struct struct__FcConstant {
    name: *FcChar8,
    object: *c_char,
    value: c_int,
}

pub type FcConstant = struct__FcConstant;

pub type enum__FcResult = c_uint;
pub static FcResultMatch: u32 = 0_u32;
pub static FcResultNoMatch: u32 = 1_u32;
pub static FcResultTypeMismatch: u32 = 2_u32;
pub static FcResultNoId: u32 = 3_u32;
pub static FcResultOutOfMemory: u32 = 4_u32;

pub type FcResult = enum__FcResult;

pub type struct__FcPattern = c_void;

pub type FcPattern = struct__FcPattern;

pub type struct__FcLangSet = c_void;

pub type FcLangSet = struct__FcLangSet;

pub struct struct__FcValue {
    _type: FcType,
    u: union_unnamed1,
}

pub type FcValue = struct__FcValue;

pub struct struct__FcFontSet {
    nfont: c_int,
    sfont: c_int,
    fonts: **FcPattern,
}

pub type FcFontSet = struct__FcFontSet;

pub struct struct__FcObjectSet {
    nobject: c_int,
    sobject: c_int,
    objects: **c_char,
}

pub type FcObjectSet = struct__FcObjectSet;

pub type enum__FcMatchKind = c_uint;
pub static FcMatchPattern: u32 = 0_u32;
pub static FcMatchFont: u32 = 1_u32;
pub static FcMatchScan: u32 = 2_u32;

pub type FcMatchKind = enum__FcMatchKind;

pub type enum__FcLangResult = c_uint;
pub static FcLangEqual: u32 = 0_u32;
pub static FcLangDifferentCountry: u32 = 1_u32;
pub static FcLangDifferentTerritory: u32 = 1_u32;
pub static FcLangDifferentLang: u32 = 2_u32;

pub type FcLangResult = enum__FcLangResult;

pub type enum__FcSetName = c_uint;
pub static FcSetSystem: u32 = 0_u32;
pub static FcSetApplication: u32 = 1_u32;

pub type FcSetName = enum__FcSetName;

pub type struct__FcAtomic = c_void;

pub type FcAtomic = struct__FcAtomic;


pub type FcEndian = c_uint;
pub static FcEndianBig: u32 = 0_u32;
pub static FcEndianLittle: u32 = 1_u32;

pub type struct__FcConfig = c_void;

pub type FcConfig = struct__FcConfig;

pub type struct__FcGlobalCache = c_void;

pub type FcFileCache = struct__FcGlobalCache;

pub type struct__FcBlanks = c_void;

pub type FcBlanks = struct__FcBlanks;

pub type struct__FcStrList = c_void;

pub type FcStrList = struct__FcStrList;

pub type struct__FcStrSet = c_void;

pub type FcStrSet = struct__FcStrSet;

pub type struct__FcCache = c_void;

pub type FcCache = struct__FcCache;

pub type union_unnamed1 = c_void /* FIXME: union type */;

#[link(name="fontconfig")]
extern {

pub fn FcBlanksCreate() -> *FcBlanks;

pub fn FcBlanksDestroy(b: *FcBlanks);

pub fn FcBlanksAdd(b: *FcBlanks, ucs4: FcChar32) -> FcBool;

pub fn FcBlanksIsMember(b: *FcBlanks, ucs4: FcChar32) -> FcBool;

pub fn FcCacheDir(c: *FcCache) -> *FcChar8;

pub fn FcCacheCopySet(c: *FcCache) -> *FcFontSet;

pub fn FcCacheSubdir(c: *FcCache, i: c_int) -> *FcChar8;

pub fn FcCacheNumSubdir(c: *FcCache) -> c_int;

pub fn FcCacheNumFont(c: *FcCache) -> c_int;

pub fn FcDirCacheUnlink(dir: *FcChar8, config: *FcConfig) -> FcBool;

pub fn FcDirCacheValid(cache_file: *FcChar8) -> FcBool;

pub fn FcConfigHome() -> *FcChar8;

pub fn FcConfigEnableHome(enable: FcBool) -> FcBool;

pub fn FcConfigFilename(url: *FcChar8) -> *FcChar8;

pub fn FcConfigCreate() -> *FcConfig;

pub fn FcConfigReference(config: *FcConfig) -> *FcConfig;

pub fn FcConfigDestroy(config: *FcConfig);

pub fn FcConfigSetCurrent(config: *FcConfig) -> FcBool;

pub fn FcConfigGetCurrent() -> *FcConfig;

pub fn FcConfigUptoDate(config: *FcConfig) -> FcBool;

pub fn FcConfigBuildFonts(config: *FcConfig) -> FcBool;

pub fn FcConfigGetFontDirs(config: *FcConfig) -> *FcStrList;

pub fn FcConfigGetConfigDirs(config: *FcConfig) -> *FcStrList;

pub fn FcConfigGetConfigFiles(config: *FcConfig) -> *FcStrList;

pub fn FcConfigGetCache(config: *FcConfig) -> *FcChar8;

pub fn FcConfigGetBlanks(config: *FcConfig) -> *FcBlanks;

pub fn FcConfigGetCacheDirs(config: *FcConfig) -> *FcStrList;

pub fn FcConfigGetRescanInterval(config: *FcConfig) -> c_int;

pub fn FcConfigSetRescanInterval(config: *FcConfig, rescanInterval: c_int) -> FcBool;

pub fn FcConfigGetFonts(config: *FcConfig, set: FcSetName) -> *FcFontSet;

pub fn FcConfigAppFontAddFile(config: *FcConfig, file: *FcChar8) -> FcBool;

pub fn FcConfigAppFontAddDir(config: *FcConfig, dir: *FcChar8) -> FcBool;

pub fn FcConfigAppFontClear(config: *FcConfig);

pub fn FcConfigSubstituteWithPat(config: *FcConfig, p: *FcPattern, p_pat: *FcPattern, kind: FcMatchKind) -> FcBool;

pub fn FcConfigSubstitute(config: *FcConfig, p: *FcPattern, kind: FcMatchKind) -> FcBool;

pub fn FcCharSetCreate() -> *FcCharSet;

pub fn FcCharSetNew() -> *FcCharSet;

pub fn FcCharSetDestroy(fcs: *FcCharSet);

pub fn FcCharSetAddChar(fcs: *FcCharSet, ucs4: FcChar32) -> FcBool;

pub fn FcCharSetCopy(src: *FcCharSet) -> *FcCharSet;

pub fn FcCharSetEqual(a: *FcCharSet, b: *FcCharSet) -> FcBool;

pub fn FcCharSetIntersect(a: *FcCharSet, b: *FcCharSet) -> *FcCharSet;

pub fn FcCharSetUnion(a: *FcCharSet, b: *FcCharSet) -> *FcCharSet;

pub fn FcCharSetSubtract(a: *FcCharSet, b: *FcCharSet) -> *FcCharSet;

pub fn FcCharSetMerge(a: *FcCharSet, b: *FcCharSet, changed: *FcBool) -> FcBool;

pub fn FcCharSetHasChar(fcs: *FcCharSet, ucs4: FcChar32) -> FcBool;

pub fn FcCharSetCount(a: *FcCharSet) -> FcChar32;

pub fn FcCharSetIntersectCount(a: *FcCharSet, b: *FcCharSet) -> FcChar32;

pub fn FcCharSetSubtractCount(a: *FcCharSet, b: *FcCharSet) -> FcChar32;

pub fn FcCharSetIsSubset(a: *FcCharSet, b: *FcCharSet) -> FcBool;

pub fn FcCharSetFirstPage(a: *FcCharSet, map: *FcChar32, next: *FcChar32) -> FcChar32;

pub fn FcCharSetNextPage(a: *FcCharSet, map: *FcChar32, next: *FcChar32) -> FcChar32;

pub fn FcCharSetCoverage(a: *FcCharSet, page: FcChar32, result: *FcChar32) -> FcChar32;

pub fn FcValuePrint(v: FcValue);

pub fn FcPatternPrint(p: *FcPattern);

pub fn FcFontSetPrint(s: *FcFontSet);

pub fn FcDefaultSubstitute(pattern: *FcPattern);

pub fn FcFileIsDir(file: *FcChar8) -> FcBool;

pub fn FcFileScan(set: *FcFontSet, dirs: *FcStrSet, cache: *FcFileCache, blanks: *FcBlanks, file: *FcChar8, force: FcBool) -> FcBool;

pub fn FcDirScan(set: *FcFontSet, dirs: *FcStrSet, cache: *FcFileCache, blanks: *FcBlanks, dir: *FcChar8, force: FcBool) -> FcBool;

pub fn FcDirSave(set: *FcFontSet, dirs: *FcStrSet, dir: *FcChar8) -> FcBool;

pub fn FcDirCacheLoad(dir: *FcChar8, config: *FcConfig, cache_file: **FcChar8) -> *FcCache;

pub fn FcDirCacheRead(dir: *FcChar8, force: FcBool, config: *FcConfig) -> *FcCache;

//pub fn FcDirCacheLoadFile(cache_file: *FcChar8, file_stat: *struct_stat) -> *FcCache;

pub fn FcDirCacheUnload(cache: *FcCache);

pub fn FcFreeTypeQuery(file: *FcChar8, id: c_int, blanks: *FcBlanks, count: *c_int) -> *FcPattern;

pub fn FcFontSetCreate() -> *FcFontSet;

pub fn FcFontSetDestroy(s: *FcFontSet);

pub fn FcFontSetAdd(s: *FcFontSet, font: *FcPattern) -> FcBool;

pub fn FcInitLoadConfig() -> *FcConfig;

pub fn FcInitLoadConfigAndFonts() -> *FcConfig;

pub fn FcInit() -> FcBool;

pub fn FcFini();

pub fn FcGetVersion() -> c_int;

pub fn FcInitReinitialize() -> FcBool;

pub fn FcInitBringUptoDate() -> FcBool;

pub fn FcGetLangs() -> *FcStrSet;

pub fn FcLangGetCharSet(lang: *FcChar8) -> *FcCharSet;

pub fn FcLangSetCreate() -> *FcLangSet;

pub fn FcLangSetDestroy(ls: *FcLangSet);

pub fn FcLangSetCopy(ls: *FcLangSet) -> *FcLangSet;

pub fn FcLangSetAdd(ls: *FcLangSet, lang: *FcChar8) -> FcBool;

pub fn FcLangSetHasLang(ls: *FcLangSet, lang: *FcChar8) -> FcLangResult;

pub fn FcLangSetCompare(lsa: *FcLangSet, lsb: *FcLangSet) -> FcLangResult;

pub fn FcLangSetContains(lsa: *FcLangSet, lsb: *FcLangSet) -> FcBool;

pub fn FcLangSetEqual(lsa: *FcLangSet, lsb: *FcLangSet) -> FcBool;

pub fn FcLangSetHash(ls: *FcLangSet) -> FcChar32;

pub fn FcLangSetGetLangs(ls: *FcLangSet) -> *FcStrSet;

pub fn FcObjectSetCreate() -> *FcObjectSet;

pub fn FcObjectSetAdd(os: *FcObjectSet, object: *c_char) -> FcBool;

pub fn FcObjectSetDestroy(os: *FcObjectSet);

//pub fn FcObjectSetVaBuild(first: *c_char, va: *__va_list_tag) -> *FcObjectSet;

pub fn FcObjectSetBuild(first: *c_char/* FIXME: variadic function */) -> *FcObjectSet;

pub fn FcFontSetList(config: *FcConfig, sets: **FcFontSet, nsets: c_int, p: *FcPattern, os: *FcObjectSet) -> *FcFontSet;

pub fn FcFontList(config: *FcConfig, p: *FcPattern, os: *FcObjectSet) -> *FcFontSet;

pub fn FcAtomicCreate(file: *FcChar8) -> *FcAtomic;

pub fn FcAtomicLock(atomic: *FcAtomic) -> FcBool;

pub fn FcAtomicNewFile(atomic: *FcAtomic) -> *FcChar8;

pub fn FcAtomicOrigFile(atomic: *FcAtomic) -> *FcChar8;

pub fn FcAtomicReplaceOrig(atomic: *FcAtomic) -> FcBool;

pub fn FcAtomicDeleteNew(atomic: *FcAtomic);

pub fn FcAtomicUnlock(atomic: *FcAtomic);

pub fn FcAtomicDestroy(atomic: *FcAtomic);

pub fn FcFontSetMatch(config: *FcConfig, sets: **FcFontSet, nsets: c_int, p: *FcPattern, result: *FcResult) -> *FcPattern;

pub fn FcFontMatch(config: *FcConfig, p: *FcPattern, result: *FcResult) -> *FcPattern;

pub fn FcFontRenderPrepare(config: *FcConfig, pat: *FcPattern, font: *FcPattern) -> *FcPattern;

pub fn FcFontSetSort(config: *FcConfig, sets: **FcFontSet, nsets: c_int, p: *FcPattern, trim: FcBool, csp: **FcCharSet, result: *FcResult) -> *FcFontSet;

pub fn FcFontSort(config: *FcConfig, p: *FcPattern, trim: FcBool, csp: **FcCharSet, result: *FcResult) -> *FcFontSet;

pub fn FcFontSetSortDestroy(fs: *FcFontSet);

pub fn FcMatrixCopy(mat: *FcMatrix) -> *FcMatrix;

pub fn FcMatrixEqual(mat1: *FcMatrix, mat2: *FcMatrix) -> FcBool;

pub fn FcMatrixMultiply(result: *FcMatrix, a: *FcMatrix, b: *FcMatrix);

pub fn FcMatrixRotate(m: *FcMatrix, c: c_double, s: c_double);

pub fn FcMatrixScale(m: *FcMatrix, sx: c_double, sy: c_double);

pub fn FcMatrixShear(m: *FcMatrix, sh: c_double, sv: c_double);

pub fn FcNameRegisterObjectTypes(types: *FcObjectType, ntype: c_int) -> FcBool;

pub fn FcNameUnregisterObjectTypes(types: *FcObjectType, ntype: c_int) -> FcBool;

pub fn FcNameGetObjectType(object: *c_char) -> *FcObjectType;

pub fn FcNameRegisterConstants(consts: *FcConstant, nconsts: c_int) -> FcBool;

pub fn FcNameUnregisterConstants(consts: *FcConstant, nconsts: c_int) -> FcBool;

pub fn FcNameGetConstant(string: *FcChar8) -> *FcConstant;

pub fn FcNameConstant(string: *FcChar8, result: *c_int) -> FcBool;

pub fn FcNameParse(name: *FcChar8) -> *FcPattern;

pub fn FcNameUnparse(pat: *FcPattern) -> *FcChar8;

pub fn FcPatternCreate() -> *FcPattern;

pub fn FcPatternDuplicate(p: *FcPattern) -> *FcPattern;

pub fn FcPatternReference(p: *FcPattern);

pub fn FcPatternFilter(p: *FcPattern, os: *FcObjectSet) -> *FcPattern;

pub fn FcValueDestroy(v: FcValue);

pub fn FcValueEqual(va: FcValue, vb: FcValue) -> FcBool;

pub fn FcValueSave(v: FcValue) -> FcValue;

pub fn FcPatternDestroy(p: *FcPattern);

pub fn FcPatternEqual(pa: *FcPattern, pb: *FcPattern) -> FcBool;

pub fn FcPatternEqualSubset(pa: *FcPattern, pb: *FcPattern, os: *FcObjectSet) -> FcBool;

pub fn FcPatternHash(p: *FcPattern) -> FcChar32;

pub fn FcPatternAdd(p: *FcPattern, object: *c_char, value: FcValue, append: FcBool) -> FcBool;

pub fn FcPatternAddWeak(p: *FcPattern, object: *c_char, value: FcValue, append: FcBool) -> FcBool;

pub fn FcPatternGet(p: *FcPattern, object: *c_char, id: c_int, v: *FcValue) -> FcResult;

pub fn FcPatternDel(p: *FcPattern, object: *c_char) -> FcBool;

pub fn FcPatternRemove(p: *FcPattern, object: *c_char, id: c_int) -> FcBool;

pub fn FcPatternAddInteger(p: *FcPattern, object: *c_char, i: c_int) -> FcBool;

pub fn FcPatternAddDouble(p: *FcPattern, object: *c_char, d: c_double) -> FcBool;

pub fn FcPatternAddString(p: *FcPattern, object: *c_char, s: *FcChar8) -> FcBool;

pub fn FcPatternAddMatrix(p: *FcPattern, object: *c_char, s: *FcMatrix) -> FcBool;

pub fn FcPatternAddCharSet(p: *FcPattern, object: *c_char, c: *FcCharSet) -> FcBool;

pub fn FcPatternAddBool(p: *FcPattern, object: *c_char, b: FcBool) -> FcBool;

pub fn FcPatternAddLangSet(p: *FcPattern, object: *c_char, ls: *FcLangSet) -> FcBool;

pub fn FcPatternGetInteger(p: *FcPattern, object: *c_char, n: c_int, i: *c_int) -> FcResult;

pub fn FcPatternGetDouble(p: *FcPattern, object: *c_char, n: c_int, d: *c_double) -> FcResult;

pub fn FcPatternGetString(p: *FcPattern, object: *c_char, n: c_int, s: **FcChar8) -> FcResult;

pub fn FcPatternGetMatrix(p: *FcPattern, object: *c_char, n: c_int, s: **FcMatrix) -> FcResult;

pub fn FcPatternGetCharSet(p: *FcPattern, object: *c_char, n: c_int, c: **FcCharSet) -> FcResult;

pub fn FcPatternGetBool(p: *FcPattern, object: *c_char, n: c_int, b: *FcBool) -> FcResult;

pub fn FcPatternGetLangSet(p: *FcPattern, object: *c_char, n: c_int, ls: **FcLangSet) -> FcResult;

//pub fn FcPatternVaBuild(p: *FcPattern, va: *__va_list_tag) -> *FcPattern;

pub fn FcPatternBuild(p: *FcPattern/* FIXME: variadic function */) -> *FcPattern;

pub fn FcPatternFormat(pat: *FcPattern, format: *FcChar8) -> *FcChar8;

pub fn FcStrCopy(s: *FcChar8) -> *FcChar8;

pub fn FcStrCopyFilename(s: *FcChar8) -> *FcChar8;

pub fn FcStrPlus(s1: *FcChar8, s2: *FcChar8) -> *FcChar8;

pub fn FcStrFree(s: *FcChar8);

pub fn FcStrDowncase(s: *FcChar8) -> *FcChar8;

pub fn FcStrCmpIgnoreCase(s1: *FcChar8, s2: *FcChar8) -> c_int;

pub fn FcStrCmp(s1: *FcChar8, s2: *FcChar8) -> c_int;

pub fn FcStrStrIgnoreCase(s1: *FcChar8, s2: *FcChar8) -> *FcChar8;

pub fn FcStrStr(s1: *FcChar8, s2: *FcChar8) -> *FcChar8;

pub fn FcUtf8ToUcs4(src_orig: *FcChar8, dst: *FcChar32, len: c_int) -> c_int;

pub fn FcUtf8Len(string: *FcChar8, len: c_int, nchar: *c_int, wchar: *c_int) -> FcBool;

pub fn FcUcs4ToUtf8(ucs4: FcChar32, dest: *FcChar8) -> c_int;

pub fn FcUtf16ToUcs4(src_orig: *FcChar8, endian: FcEndian, dst: *FcChar32, len: c_int) -> c_int;

pub fn FcUtf16Len(string: *FcChar8, endian: FcEndian, len: c_int, nchar: *c_int, wchar: *c_int) -> FcBool;

pub fn FcStrDirname(file: *FcChar8) -> *FcChar8;

pub fn FcStrBasename(file: *FcChar8) -> *FcChar8;

pub fn FcStrSetCreate() -> *FcStrSet;

pub fn FcStrSetMember(set: *FcStrSet, s: *FcChar8) -> FcBool;

pub fn FcStrSetEqual(sa: *FcStrSet, sb: *FcStrSet) -> FcBool;

pub fn FcStrSetAdd(set: *FcStrSet, s: *FcChar8) -> FcBool;

pub fn FcStrSetAddFilename(set: *FcStrSet, s: *FcChar8) -> FcBool;

pub fn FcStrSetDel(set: *FcStrSet, s: *FcChar8) -> FcBool;

pub fn FcStrSetDestroy(set: *FcStrSet);

pub fn FcStrListCreate(set: *FcStrSet) -> *FcStrList;

pub fn FcStrListNext(list: *FcStrList) -> *FcChar8;

pub fn FcStrListDone(list: *FcStrList);

pub fn FcConfigParseAndLoad(config: *FcConfig, file: *FcChar8, complain: FcBool) -> FcBool;

}
