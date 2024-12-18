import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../providers/atmos_provider.dart';
import '../widgets/sensor_card.dart';
import '../widgets/relay_card.dart';
import '../widgets/atmos_graph.dart';

class HomeScreen extends StatefulWidget {
  const HomeScreen({super.key});

  @override
  HomeScreenState createState() =>
      HomeScreenState(); // Changed to HomeScreenState
}

class HomeScreenState extends State<HomeScreen> {
  // Changed to HomeScreenState
  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) {
      Provider.of<AtmosProvider>(context, listen: false).fetchAllData();
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Atmos Control',
            style: TextStyle(fontWeight: FontWeight.bold)),
      ),
      body: Consumer<AtmosProvider>(
        builder: (context, atmosProvider, child) {
          final atmosData = atmosProvider.atmosData;
          if (atmosData == null) {
            return const Center(child: CircularProgressIndicator());
          }
          return RefreshIndicator(
            onRefresh: () => atmosProvider.fetchAllData(),
            child: ListView(
              padding: const EdgeInsets.all(16),
              children: [
                Row(
                  children: [
                    Expanded(
                      child: SensorCard(
                        title: 'Temperature',
                        value: atmosData.averageTemp.toStringAsFixed(1),
                        unit: '°C',
                        icon: Icons.thermostat,
                      ),
                    ),
                    const SizedBox(width: 16),
                    Expanded(
                      child: SensorCard(
                        title: 'Humidity',
                        value: atmosData.averageHumidity.toStringAsFixed(1),
                        unit: '%',
                        icon: Icons.water_drop,
                      ),
                    ),
                  ],
                ),
                const SizedBox(height: 16),
                Text(
                  'Appliances',
                  style: Theme.of(context).textTheme.titleLarge,
                ),
                const SizedBox(height: 8),
                RelayCard(
                  title: 'Fridge',
                  relayStatus: atmosProvider.getRelayStatus('fridge'),
                  onToggle: () => atmosProvider.changeRelayStatus('fridge'),
                  icon: Icons.ac_unit,
                ),
                RelayCard(
                  title: 'Humidifier',
                  relayStatus: atmosProvider.getRelayStatus('humidifier'),
                  onToggle: () => atmosProvider.changeRelayStatus('humidifier'),
                  icon: Icons.cloud,
                ),
                RelayCard(
                  title: 'Dehumidifier',
                  relayStatus: atmosProvider.getRelayStatus('dehumidifier'),
                  onToggle: () =>
                      atmosProvider.changeRelayStatus('dehumidifier'),
                  icon: Icons.water_drop_outlined,
                ),
                RelayCard(
                  title: 'Heater',
                  relayStatus: atmosProvider.getRelayStatus('heater'),
                  onToggle: () => atmosProvider.changeRelayStatus('heater'),
                  icon: Icons.ac_unit,
                ),
                RelayCard(
                  title: 'Ventilator',
                  relayStatus: atmosProvider.getRelayStatus('ventilator'),
                  onToggle: () => atmosProvider.changeRelayStatus('ventilator'),
                  icon: Icons.air,
                ),
                const SizedBox(height: 16),
                Text(
                  'Last sensor poll: ${atmosData.formattedLastReadingTime()}',
                  style: Theme.of(context).textTheme.bodySmall,
                  textAlign: TextAlign.center,
                ),
                const AtmosGraph(),
              ],
            ),
          );
        },
      ),
    );
  }
}
