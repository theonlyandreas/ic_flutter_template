#import "IcFfiPlugin.h"
#if __has_include(<ic_ffi/ic_ffi-Swift.h>)
#import <ic_ffi/ic_ffi-Swift.h>
#else
// Support project import fallback if the generated compatibility header
// is not copied when this plugin is created as a library.
// https://forums.swift.org/t/swift-static-libraries-dont-copy-generated-objective-c-header/19816
#import "ic_ffi-Swift.h"
#endif

@implementation IcFfiPlugin
+ (void)registerWithRegistrar:(NSObject<FlutterPluginRegistrar>*)registrar {
  [SwiftIcFfiPlugin registerWithRegistrar:registrar];
}
@end
