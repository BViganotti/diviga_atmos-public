import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'providers/atmos_provider.dart';
import 'services/atmos_service.dart';
import 'screens/home_screen.dart';

void main() {
  runApp(MyApp());
}

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return ChangeNotifierProvider(
      create: (context) => AtmosProvider(
        AtmosService(baseUrl: 'http://0.0.0.0:8080'),
      ),
      child: MaterialApp(
        title: 'Atmos Control',
        theme: ThemeData(
          primarySwatch: Colors.blue,
          visualDensity: VisualDensity.adaptivePlatformDensity,
        ),
        home: HomeScreen(),
      ),
    );
  }
}
