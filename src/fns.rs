/*
   The code is automatically generated by the genBind tool.
   Author: ying32
   https://github.com/ying32
*/

#![allow(non_snake_case)]
#![allow(unused_unsafe)]
#![allow(dead_code)]

use lclapi;
use std::borrow::Cow;
use std::ffi::{CStr, CString};
use types::*;
use vcl::{TControl, TWinControl, IObject, IComponent, IStrings, IStream, TClipboard};

pub fn GetFPStringArrayMember<'a>(ptr: usize, index: isize) -> Cow<'a, str> {
    return ToRustString(unsafe { lclapi::DGetStringArrOf(ptr, index) });
}

pub fn GetLibResourceItem(index: i32) -> TResItem {
    let mut result = TResItem::empty();
    unsafe {
        lclapi::DGetLibResourceItem(index, &mut result);
    }
    return result;
}

pub fn StringToGUID(guidStr: &str) -> TGUID {
    let mut result = TGUID::empty();
    unsafe { lclapi::DStringToGUID(to_CString!(guidStr), &mut result) }
    return result;
}

pub fn CreateGUID() -> TGUID {
    let mut result = TGUID::empty();
    unsafe { lclapi::DCreateGUID(&mut result) }
    return result;
}

#[cfg(target_os = "linux")]
pub fn GdkWindow_GetXId(AW: PGdkWindow) -> TXId {
    let mut result: TXID = 0;
    unsafe { lclapi::GdkWindow_GetXId(AW, &mut result) }
    return result;
}

pub fn SelectDirectory<'a>(Options: TSelectDirOpts, HelpCtx: i32) -> (bool, Cow<'a, str>) {
    let mut cstr = to_CString!("");
    let result = unsafe { lclapi::DSelectDirectory1(&mut cstr, Options, HelpCtx) };
    return (result, ToRustString(cstr), );
}

pub fn SelectDirectory2<'a>(Caption: &str, Root: &str, AShowHidden: bool) -> (bool, Cow<'a, str>) {
    let mut cstr = to_CString!("");
    let result = unsafe { lclapi::DSelectDirectory2(to_CString!(Caption), to_CString!(Root), AShowHidden, &mut cstr) };
    return (result, ToRustString(cstr), );
}

pub fn InputQuery<'a>(ACaption: &str, APrompt: &str, Value: &str)-> (bool, Cow<'a, str>) {
    let mut cstr = to_CString!("");
    let result = unsafe { lclapi::DInputQuery(to_CString!(ACaption), to_CString!(APrompt), to_CString!(Value), &mut cstr) };
    return (result, ToRustString(cstr), );
}

pub fn ToRustString<'a>(s: *const i8) -> Cow<'a, str> {
    if s == 0 as *const i8 {
        return Cow::Owned(String::from(""));
    }
    return unsafe { CStr::from_ptr(s).to_string_lossy() };
}

#[inline]
pub fn toCString(s: &str) -> *const i8 {
    return CString::new(s).unwrap().as_ptr();
}

pub fn WindowFromPoint(point: &TPoint) -> HWND {
    unsafe {
         let mut ps0 = point.clone();
         return lclapi::DWindowFromPoint(&mut ps0);
    }
}


pub fn LibStringEncoding() -> TStringEncoding {
    unsafe { return lclapi::DLibStringEncoding()}
}

pub fn LibVersion() -> u32 {
    unsafe { return lclapi::DLibVersion()}
}

pub fn SysLocale(aInfo: *mut TSysLocale) {
    unsafe { lclapi::DSysLocale(aInfo)}
}

pub fn TextToShortCut(aText: &str) -> TShortCut {
    unsafe { return lclapi::DTextToShortCut(to_CString!(aText))}
}

pub fn ShortCutToText<'a>(aVal: TShortCut) -> Cow<'a, str> {
    unsafe { return ToRustString(lclapi::DShortCutToText(aVal))}
}

pub fn SetClipboard(aNewClipboard: &TClipboard) -> TClipboard {
    unsafe { return TClipboard::from(lclapi::DSetClipboard(aNewClipboard.Instance()))}
}

pub fn SendMessage(hWd: HWND, msg: u32, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
    unsafe { return lclapi::DSendMessage(hWd, msg, wParam, lParam)}
}

pub fn PostMessage(hWd: HWND, msg: u32, wParam: WPARAM, lParam: LPARAM) -> bool {
    unsafe { return lclapi::DPostMessage(hWd, msg, wParam, lParam)}
}

pub fn IsIconic(hWnd: HWND) -> bool {
    unsafe { return lclapi::DIsIconic(hWnd)}
}

pub fn IsWindow(hWnd: HWND) -> bool {
    unsafe { return lclapi::DIsWindow(hWnd)}
}

pub fn IsZoomed(hWnd: HWND) -> bool {
    unsafe { return lclapi::DIsZoomed(hWnd)}
}

pub fn IsWindowVisible(hWnd: HWND) -> bool {
    unsafe { return lclapi::DIsWindowVisible(hWnd)}
}

pub fn GetDC(hWnd: HWND) -> HDC {
    unsafe { return lclapi::DGetDC(hWnd)}
}

pub fn ReleaseDC(hWnd: HWND, dc: HDC) -> i32 {
    unsafe { return lclapi::DReleaseDC(hWnd, dc)}
}

pub fn SetForegroundWindow(hWnd: HWND) -> bool {
    unsafe { return lclapi::DSetForegroundWindow(hWnd)}
}

pub fn RegisterClipboardFormat(aFormat: &str) -> TClipboardFormat {
    unsafe { return lclapi::DRegisterClipboardFormat(to_CString!(aFormat))}
}

pub fn ShowMessage(aMsg: &str) {
    unsafe { lclapi::DShowMessage(to_CString!(aMsg))}
}

pub fn GetMainInstance() -> HINST {
    unsafe { return lclapi::DGetMainInstance()}
}

pub fn MessageDlg(msg: &str, dlgType: TMsgDlgType, buttons: TMsgDlgButtons, helpCtx: i32) -> i32 {
    unsafe { return lclapi::DMessageDlg(to_CString!(msg), dlgType, buttons, helpCtx)}
}

pub fn MainThreadId() -> TThreadID {
    unsafe { return lclapi::DMainThreadId()}
}

pub fn CurrentThreadId() -> TThreadID {
    unsafe { return lclapi::DCurrentThreadId()}
}

pub fn SysOpen(fileName: &str) {
    unsafe { lclapi::DSysOpen(to_CString!(fileName))}
}

pub fn ExtractFilePath<'a>(aFileName: &str) -> Cow<'a, str> {
    unsafe { return ToRustString(lclapi::DExtractFilePath(to_CString!(aFileName)))}
}

pub fn FileExists(aFileName: &str) -> bool {
    unsafe { return lclapi::DFileExists(to_CString!(aFileName))}
}

pub fn InputBox<'a>(aCaption: &str, aPrompt: &str, aDefault: &str) -> Cow<'a, str> {
    unsafe { return ToRustString(lclapi::DInputBox(to_CString!(aCaption), to_CString!(aPrompt), to_CString!(aDefault)))}
}

pub fn PasswordBox<'a>(aCaption: &str, aPrompt: &str) -> Cow<'a, str> {
    unsafe { return ToRustString(lclapi::DPasswordBox(to_CString!(aCaption), to_CString!(aPrompt)))}
}

pub fn InputCombo(aCaption: &str, aPrompt: &str, aList: &dyn IStrings) -> i32 {
    unsafe { return lclapi::DInputCombo(to_CString!(aCaption), to_CString!(aPrompt), aList.Instance())}
}

pub fn InputComboEx<'a>(aCaption: &str, aPrompt: &str, aList: &dyn IStrings, allowCustomText: bool) -> Cow<'a, str> {
    unsafe { return ToRustString(lclapi::DInputComboEx(to_CString!(aCaption), to_CString!(aPrompt), aList.Instance(), allowCustomText))}
}

#[cfg(target_os = "windows")]
pub fn CreateURLShortCut(aDestPath: &str, aShortCutName: &str, aURL: &str) {
    unsafe { lclapi::DCreateURLShortCut(to_CString!(aDestPath), to_CString!(aShortCutName), to_CString!(aURL))}
}

#[cfg(target_os = "windows")]
pub fn CreateShortCut(aDestPath: &str, aShortCutName: &str, aSrcFileName: &str, aIconFileName: &str, aDescription: &str, aCmdArgs: &str) -> bool {
    unsafe { return lclapi::DCreateShortCut(to_CString!(aDestPath), to_CString!(aShortCutName), to_CString!(aSrcFileName), to_CString!(aIconFileName), to_CString!(aDescription), to_CString!(aCmdArgs))}
}

pub fn SetPropertyValue(instance: &dyn IObject, propName: &str, value: &str) {
    unsafe { lclapi::DSetPropertyValue(instance.Instance(), to_CString!(propName), to_CString!(value))}
}

pub fn SetPropertySecValue(instance: &dyn IObject, propName: &str, secPropName: &str, value: &str) {
    unsafe { lclapi::DSetPropertySecValue(instance.Instance(), to_CString!(propName), to_CString!(secPropName), to_CString!(value))}
}

pub fn GUIDToString<'a>(aGUID: &TGUID) -> Cow<'a, str> {
    let mut ps0 = aGUID.clone();
    unsafe { return ToRustString(lclapi::DGUIDToString(&mut ps0))}
}

pub fn LibAbout<'a>() -> Cow<'a, str> {
    unsafe { return ToRustString(lclapi::DLibAbout())}
}

pub fn GetLibResourceCount() -> i32 {
    unsafe { return lclapi::DGetLibResourceCount()}
}

pub fn ModifyLibResource(aPtr: usize, aValue: &str) {
    unsafe { lclapi::DModifyLibResource(aPtr, to_CString!(aValue))}
}

pub fn GetLibType() -> TLibType {
    unsafe { return lclapi::DGetLibType()}
}

pub fn InitGoDll(aMainThreadId: TThreadID) {
    unsafe { lclapi::DInitGoDll(aMainThreadId)}
}

pub fn FindControl(aHandle: HWND) -> TWinControl {
    unsafe { return TWinControl::from(lclapi::DFindControl(aHandle))}
}

pub fn FindLCLControl(aScreenPos: &TPoint) -> TControl {
    let mut ps0 = aScreenPos.clone();
    unsafe { return TControl::from(lclapi::DFindLCLControl(&mut ps0))}
}

pub fn FindOwnerControl(handle: HWND) -> TWinControl {
    unsafe { return TWinControl::from(lclapi::DFindOwnerControl(handle))}
}

pub fn FindControlAtPosition(aPosition: &TPoint, allowDisabled: bool) -> TControl {
    let mut ps0 = aPosition.clone();
    unsafe { return TControl::from(lclapi::DFindControlAtPosition(&mut ps0, allowDisabled))}
}

pub fn FindLCLWindow(aScreenPos: &TPoint, allowDisabled: bool) -> TWinControl {
    let mut ps0 = aScreenPos.clone();
    unsafe { return TWinControl::from(lclapi::DFindLCLWindow(&mut ps0, allowDisabled))}
}

pub fn FindDragTarget(aPosition: &TPoint, allowDisabled: bool) -> TControl {
    let mut ps0 = aPosition.clone();
    unsafe { return TControl::from(lclapi::DFindDragTarget(&mut ps0, allowDisabled))}
}

#[cfg(target_os = "linux")]
pub fn GtkWidget_GetGtkFixed(ah: HWND) -> PGtkFixed {
    unsafe { return lclapi::GtkWidget_GetGtkFixed(ah)}
}

#[cfg(target_os = "linux")]
pub fn GdkWindow_FromForm(aForm: &TForm) -> PGdkWindow {
    unsafe { return lclapi::GdkWindow_FromForm(aForm.Instance())}
}

#[cfg(target_os = "linux")]
pub fn GtkWidget_Window(ah: HWND) -> PGdkWindow {
    unsafe { return lclapi::GtkWidget_Window(ah)}
}

#[cfg(target_os = "macos")]
pub fn NSWindow_FromForm(aForm: &TForm) -> MyNSWindow {
    unsafe { return lclapi::NSWindow_FromForm(aForm.Instance())}
}

pub fn ResFormLoadFromResourceName(aInstance: usize, aResName: &str, aRoot: &dyn IComponent) {
    unsafe { lclapi::ResFormLoadFromResourceName(aInstance, to_CString!(aResName), aRoot.Instance())}
}

pub fn ResFormLoadFromFile(aFileName: &str, aRoot: &dyn IComponent) {
    unsafe { lclapi::ResFormLoadFromFile(to_CString!(aFileName), aRoot.Instance())}
}

pub fn ResFormLoadFromStream(aStream: &dyn IStream, aRoot: &dyn IComponent) {
    unsafe { lclapi::ResFormLoadFromStream(aStream.Instance(), aRoot.Instance())}
}

pub fn ResFormRegisterFormResource(aClassName: &str, aData: usize, aLen: i32) -> bool {
    unsafe { return lclapi::ResFormRegisterFormResource(to_CString!(aClassName), aData, aLen)}
}

pub fn ResFormLoadFromClassName(aClassName: &str, aRoot: &dyn IComponent) -> bool {
    unsafe { return lclapi::ResFormLoadFromClassName(to_CString!(aClassName), aRoot.Instance())}
}
