import 'package:flutter/material.dart';
import 'package:fpwinrs_ui/native.dart';

void main() {
  api.init();
  runApp(const FastPairApp());
}

class FastPairApp extends StatelessWidget {
  const FastPairApp({Key? key}) : super(key: key);
  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Fast Pair',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: const HomePage(),
    );
  }
}

class HomePage extends StatelessWidget {
  const HomePage({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("Fast Pair"),
      ),
      body: Center(
        child: StreamBuilder(
          // All Rust functions are called as Future's
          stream: api.eventStream(), // The Rust function we are calling.
          builder: (context, data) {
            if (data.hasData) {
              return Text(data.data!); // The string to display
            }
            return const Center(
              child: CircularProgressIndicator(),
            );
          },
        ),
      ),
    );
  }
}
