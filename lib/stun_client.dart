
import 'dart:async';

import 'package:flutter/services.dart';

class StunClient {
  static const MethodChannel _channel = MethodChannel('stun_client');

  static Future<String?> get platformVersion async {
    final String? version = await _channel.invokeMethod('getPlatformVersion');
    return version;
  }
}
