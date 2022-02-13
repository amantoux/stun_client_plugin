#import "StunClientPlugin.h"
#if __has_include(<stun_client/stun_client-Swift.h>)
#import <stun_client/stun_client-Swift.h>
#else
// Support project import fallback if the generated compatibility header
// is not copied when this plugin is created as a library.
// https://forums.swift.org/t/swift-static-libraries-dont-copy-generated-objective-c-header/19816
#import "stun_client-Swift.h"
#endif

@implementation StunClientPlugin
+ (void)registerWithRegistrar:(NSObject<FlutterPluginRegistrar>*)registrar {
  [SwiftStunClientPlugin registerWithRegistrar:registrar];
}
@end
