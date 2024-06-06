import 'dart:io' as io;
import 'dart:ffi';

String? dynamicLibPath;
final rustLibrary = loadRustLibrary();

void setDynamicLibPath(String? path) {
  dynamicLibPath = path;
}

DynamicLibrary loadRustLibrary() {
  // Use provided dynamic library path if possible.
  final path = dynamicLibPath;
  if (path != null) {
    return DynamicLibrary.open(path);
  }

  // Otherewise, use the default path.
  if (io.Platform.isLinux) {
    try {
      return DynamicLibrary.open('libhub.so');
    } catch (e) {
      print("========$e");
      throw UnsupportedError('This operating system is not supported.');
    }
  } else if (io.Platform.isAndroid) {
    try {
      return DynamicLibrary.open('libhub.so');
    } catch (e) {
      print("========$e");
      throw UnsupportedError('This operating system is not supported.');
    }
  } else if (io.Platform.isWindows) {
    try {
      return DynamicLibrary.open('hub.dll');
    } catch (e) {
      print("========$e");
      throw UnsupportedError('This operating system is not supported.');
    }
  } else if (io.Platform.isIOS) {
    try {
      return DynamicLibrary.open('rinf.framework/rinf');
    } catch (e) {
      print("========$e");
      throw UnsupportedError('This operating system is not supported.');
    }
  } else if (io.Platform.isMacOS) {
    try {
      return DynamicLibrary.open('rinf.framework/rinf');
    } catch (e) {
      print("========$e");
      throw UnsupportedError('This operating system is not supported.');
    }
  } else {
    throw UnsupportedError('This operating system is not supported.');
  }
}
