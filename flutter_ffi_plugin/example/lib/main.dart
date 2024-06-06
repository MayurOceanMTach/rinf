import 'dart:ui';
import 'package:example_app/messages/counter_number.pb.dart';
import 'package:example_app/messages/generated.dart';
import 'package:flutter/material.dart';

void main() async {
  // Wait for Rust initialization to be completed first.
  await initializeRust();
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  final _appLifecycleListener = AppLifecycleListener(
    onExitRequested: () async {
      await finalizeRust();
      return AppExitResponse.exit;
    },
  );

  @override
  void dispose() {
    _appLifecycleListener.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Rinf Demo',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(
          seedColor: Colors.blueGrey,
          brightness: MediaQuery.platformBrightnessOf(context),
        ),
        useMaterial3: true,
      ),
      home: MyHomePage(),
    );
  }
}

class MyHomePage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            StreamBuilder(
              stream: MyPreciousData.rustSignalStream,
              builder: (context, snapshot) {
                print(snapshot.data?.message.inputString);
                final signan = snapshot.data;
                if (signan != null && snapshot.connectionState == ConnectionState.active) {
                  print(signan.message.inputString);
                  return Text(signan.message.inputString);
                }
                print(snapshot);
                return const SizedBox.shrink();
              },
            ),
            ElevatedButton(
              onPressed: () async {
                MyPreciousData1(
                  inputNumbers: [3, 4, 5],
                  inputString: 'Zero-cost abstraction',
                ).sendSignalToRust(); // GENERATED
              },
              child: Text("Send a Signal from Dart to Rust"),
            ),
          ],
        ),
      ),
    );
  }
}
