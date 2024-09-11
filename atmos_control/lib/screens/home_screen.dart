import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../providers/atmos_provider.dart';
import '../widgets/sensor_card.dart';
import '../widgets/relay_card.dart';

class HomeScreen extends StatefulWidget {
  @override
  _HomeScreenState createState() => _HomeScreenState();
}

class _HomeScreenState extends State<HomeScreen> {
  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) {
      Provider.of<AtmosProvider>(context, listen: false).fetchAtmosData();
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Atmos Control'),
        actions: [
          IconButton(
            icon: Icon(Icons.settings),
            onPressed: () {
              // TODO: Navigate to settings screen
            },
          ),
        ],
      ),
      body: Consumer<AtmosProvider>(
        builder: (context, provider, child) {
          if (provider.atmosData == null) {
            return Center(child: CircularProgressIndicator());
          }
          return RefreshIndicator(
            onRefresh: () => provider.fetchAtmosData(),
            child: ListView(
              padding: EdgeInsets.all(16),
              children: [
                SensorCard(
                  title: 'Average Temperature',
                  value: provider.atmosData!.averageTemp,
                  unit: '°C',
                ),
                SensorCard(
                  title: 'Average Humidity',
                  value: provider.atmosData!.averageHumidity,
                  unit: '%',
                ),
                RelayCard(
                  title: 'Fridge',
                  isOn: provider.atmosData!.fridgeStatus,
                  onToggle: () => provider.toggleRelay('fridge'),
                ),
                RelayCard(
                  title: 'Humidifier',
                  isOn: provider.atmosData!.humidifierStatus,
                  onToggle: () => provider.toggleRelay('humidifier'),
                ),
                RelayCard(
                  title: 'Dehumidifier',
                  isOn: provider.atmosData!.dehumidifierStatus,
                  onToggle: () => provider.toggleRelay('dehumidifier'),
                ),
              ],
            ),
          );
        },
      ),
    );
  }
}
