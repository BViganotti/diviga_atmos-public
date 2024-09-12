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
        title: Text('Atmos Control', style: TextStyle(fontWeight: FontWeight.bold)),
      ),
      body: Consumer<AtmosProvider>(
        builder: (context, atmosProvider, child) {
          final atmosData = atmosProvider.atmosData;
          if (atmosData == null) {
            return Center(child: CircularProgressIndicator());
          }
          return RefreshIndicator(
            onRefresh: () => atmosProvider.fetchAtmosData(),
            child: ListView(
              padding: EdgeInsets.all(16),
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
                    SizedBox(width: 16),
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
                SizedBox(height: 16),
                Text(
                  'Appliances',
                  style: Theme.of(context).textTheme.titleLarge,
                ),
                SizedBox(height: 8),
                RelayCard(
                  title: 'Fridge',
                  status: atmosData.fridgeStatus ? 'On' : 'Off',
                  onToggle: () => atmosProvider.toggleRelay('fridge'),
                  icon: Icons.ac_unit,
                ),
                RelayCard(
                  title: 'Humidifier',
                  status: atmosData.humidifierStatus ? 'On' : 'Off',
                  onToggle: () => atmosProvider.toggleRelay('humidifier'),
                  icon: Icons.cloud,
                ),
                RelayCard(
                  title: 'Dehumidifier',
                  status: atmosData.dehumidifierStatus ? 'On' : 'Off',
                  onToggle: () => atmosProvider.toggleRelay('dehumidifier'),
                  icon: Icons.water_drop_outlined,
                ),
                RelayCard(
                  title: 'Heater',
                  status: atmosData.heaterStatus ? 'On' : 'Off',
                  onToggle: () => atmosProvider.toggleRelay('heater'),
                  icon: Icons.ac_unit,
                ),
                RelayCard(
                  title: 'Ventilator',
                  status: atmosData.ventilatorStatus ? 'On' : 'Off',
                  onToggle: () => atmosProvider.toggleRelay('ventilator'),
                  icon: Icons.air,
                ),
                SizedBox(height: 16),
                Text(
                  'Last sensor poll: ${atmosData.formattedLastReadingTime()}',
                  style: Theme.of(context).textTheme.bodySmall,
                  textAlign: TextAlign.center,
                ),
              ],
            ),
          );
        },
      ),
    );
  }
}
