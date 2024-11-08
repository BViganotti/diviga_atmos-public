import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'providers/atmos_provider.dart';
import 'services/atmos_service.dart';
import 'screens/home_screen.dart';

void main() {
  runApp(
    ChangeNotifierProvider(
      create: (context) =>
          AtmosProvider(AtmosService('http://192.168.0.216:8080')),
      child: const MyApp(),
    ),
  );
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return ChangeNotifierProvider(
      create: (context) => AtmosProvider(
        AtmosService('http://192.168.0.216:8080'),
      ),
      child: MaterialApp(
        title: 'Atmos Control',
        theme: ThemeData.dark().copyWith(
          primaryColor: Colors.teal,
          scaffoldBackgroundColor: const Color(0xFF121212),
          cardColor: const Color(0xFF1E1E1E),
          appBarTheme: const AppBarTheme(
            backgroundColor: Color(0xFF1E1E1E),
            elevation: 0,
          ),
        ),
        home: const HomeScreen(),
      ),
    );
  }
}
