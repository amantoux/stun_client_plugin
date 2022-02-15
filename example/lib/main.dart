import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:stun_client/stun_client.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({Key? key}) : super(key: key);

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  String xorMappedAddress = 'Unknown';

  @override
  void initState() {
    super.initState();
    initXorMappedAddress();
  }

  Future<void> initXorMappedAddress() async {
    String mappedAddress;
    // Platform messages may fail, so we use a try/catch PlatformException.
    // We also handle the message potentially returning null.
    try {
      mappedAddress = await StunClient.getXorMappedAddress(
              'plato-test.mantoux.org:3478',
              '3522',
              Options(const Duration(seconds: 10), "stunc"));
    } catch (e) {
      mappedAddress = 'Failed to get mapped address. $e';
    }

    // If the widget was removed from the tree while the asynchronous platform
    // message was in flight, we want to discard the reply rather than calling
    // setState to update our non-existent appearance.
    if (!mounted) return;

    setState(() {
      xorMappedAddress = mappedAddress;
    });
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: const Text('Plugin example app'),
        ),
        body: Center(
          child: Text('Running on: $xorMappedAddress\n'),
        ),
      ),
    );
  }
}
