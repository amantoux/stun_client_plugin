import Flutter
import UIKit

public class SwiftStunClientPlugin: NSObject, FlutterPlugin {
  public static func register(with registrar: FlutterPluginRegistrar) {
    let channel = FlutterMethodChannel(name: "stun_client", binaryMessenger: registrar.messenger())
    let instance = SwiftStunClientPlugin()
    registrar.addMethodCallDelegate(instance, channel: channel)
  }

  public func handle(_ call: FlutterMethodCall, result: @escaping FlutterResult) {
    result("iOS " + UIDevice.current.systemVersion)
  }

  public func dummyMethodToEnforceBundling() {
    var duration = CDuration(secs: 10, nanos: 0)
    let durationPtr = UnsafePointer<CDuration>(&duration)
    let options = COptions(timeout: durationPtr, software: "ok")
    // This will never be executed
    get_xor_mapped_address("", options);
  }
}
