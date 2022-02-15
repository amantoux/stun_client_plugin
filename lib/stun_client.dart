import 'dart:async';
import 'dart:ffi';
import 'dart:io';

import 'package:ffi/ffi.dart';

class CDuration extends Struct {

  CDuration() : super();

  @Int64()
  external int secs;

  @Int32()
  external int nanos;
}

extension ToNative on Duration {
  Pointer<CDuration> toNative() {
    final Pointer<CDuration> cDurationPtr = calloc();
    cDurationPtr.ref
      ..secs = inSeconds
      ..nanos = 0;
    return cDurationPtr;
  }
}

class COptions extends Struct {
  external Pointer<CDuration> timeout;

  external Pointer<Utf8> software;
}

class CResponse extends Struct {
  @Int32()
  external int status;

  external Pointer<Utf8> value;

  external Pointer<Utf8> error;

  Response toNative() {
    return Response(status, value.address == nullptr.address ? null : value.toDartString(), error.address == nullptr.address ? null : error.toDartString());
  }
}

class Response {
  final int status;
  final String? value;
  final String? error;

  Response(this.status, this.value, this.error);
}

class Options {
  final Duration timeout;
  final String? software;

  Options(this.timeout, this.software);

  Pointer<COptions> toNativeOptions() {
    final Pointer<COptions> cOptionsPtr = calloc();
    final COptions cOptions = cOptionsPtr.ref;
    if (software == null) {
      final Pointer<Void> empty = calloc();
      cOptions.software = empty.cast();
    } else {
      cOptions.software = software!.toNativeUtf8();
    }
    cOptions.timeout = timeout.toNative();
    return cOptionsPtr;
  }
}

final stunClient = Platform.isAndroid
    ? DynamicLibrary.open("libstunc.so")
    : DynamicLibrary.process();

typedef StunClientGetXorMapped = CResponse Function(Pointer<Utf8>, Pointer<
    Utf8>, COptions);
typedef StunClientGetXorMappedFFI = CResponse Function(Pointer<Utf8>, Pointer<
    Utf8>, COptions);

final StunClientGetXorMapped getXorMapped = stunClient
    .lookup<NativeFunction<StunClientGetXorMappedFFI>>("get_xor_mapped_address")
    .asFunction();

class StunClient {
  static Future<String> getXorMappedAddress(String stunIpPort, String localPort,
      Options stunOptions) async {
    return Future<String>.microtask(() {
      final response = getXorMapped(
          stunIpPort.toNativeUtf8(), localPort.toNativeUtf8(),
          stunOptions.toNativeOptions().ref).toNative();
      if (response.status < 0) {
        throw response.error!;
      }
      return response.value!;
    });
  }
}
