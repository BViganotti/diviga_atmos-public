import 'package:flutter/material.dart';
import 'package:fl_chart/fl_chart.dart';
import 'package:provider/provider.dart';
import '../providers/atmos_provider.dart';

class AtmosGraph extends StatefulWidget {
  const AtmosGraph({super.key});

  @override
  _AtmosGraphState createState() => _AtmosGraphState();
}

class _AtmosGraphState extends State<AtmosGraph> {
  List<FlSpot> temperatureSpots = [];
  List<FlSpot> humiditySpots = [];
  String selectedTimeRange = 'Today';

  @override
  void initState() {
    super.initState();
    fetchData();
  }

  Future<void> fetchData() async {
    final atmosProvider = Provider.of<AtmosProvider>(context, listen: false);
    try {
      final data = await atmosProvider.fetchAtmosHistory(selectedTimeRange);
      print('Raw data received: $data');

      setState(() {
        temperatureSpots = data.asMap().entries.map((entry) {
          final index = entry.key.toDouble();
          final item = entry.value;
          print(
              'Temperature data point: index=$index, value=${item.averageTemp}');
          return FlSpot(index, item.averageTemp);
        }).toList();

        humiditySpots = data.asMap().entries.map((entry) {
          final index = entry.key.toDouble();
          final item = entry.value;
          print(
              'Humidity data point: index=$index, value=${item.averageHumidity}');
          return FlSpot(index, item.averageHumidity);
        }).toList();
      });

      print('Processed temperature spots: $temperatureSpots');
      print('Processed humidity spots: $humiditySpots');
    } catch (e) {
      print('Error fetching atmosphere data: $e');
    }
  }

  @override
  Widget build(BuildContext context) {
    return Card(
      elevation: 4,
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
      child: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          children: [
            DropdownButton<String>(
              value: selectedTimeRange,
              items: ['Today', 'Week', 'Month'].map((String value) {
                return DropdownMenuItem<String>(
                  value: value,
                  child: Text(value),
                );
              }).toList(),
              onChanged: (String? newValue) {
                setState(() {
                  selectedTimeRange = newValue!;
                  fetchData();
                });
              },
            ),
            const SizedBox(height: 16),
            SizedBox(
              height: 200,
              child: LineChart(
                LineChartData(
                  minY: 0,
                  maxY: 100,
                  gridData: const FlGridData(show: false),
                  titlesData: FlTitlesData(
                    leftTitles: AxisTitles(
                      sideTitles: SideTitles(
                        showTitles: true,
                        getTitlesWidget: (value, meta) {
                          return Text('${value.toStringAsFixed(0)}°C',
                              style: const TextStyle(
                                  fontSize: 10, color: Colors.red));
                        },
                        reservedSize: 30,
                      ),
                    ),
                    rightTitles: AxisTitles(
                      sideTitles: SideTitles(
                        showTitles: true,
                        getTitlesWidget: (value, meta) {
                          return Text('${value.toStringAsFixed(0)}%',
                              style: const TextStyle(
                                  fontSize: 10, color: Colors.blue));
                        },
                        reservedSize: 30,
                      ),
                    ),
                    bottomTitles: const AxisTitles(
                        sideTitles: SideTitles(showTitles: false)),
                    topTitles: const AxisTitles(
                        sideTitles: SideTitles(showTitles: false)),
                  ),
                  borderData: FlBorderData(show: false),
                  lineBarsData: [
                    LineChartBarData(
                      spots: temperatureSpots
                          .map((spot) => FlSpot(spot.x, spot.y * 2.5))
                          .toList(),
                      isCurved: true,
                      color: Colors.red,
                      barWidth: 3,
                      isStrokeCapRound: true,
                      dotData: const FlDotData(show: false),
                    ),
                    LineChartBarData(
                      spots: humiditySpots,
                      isCurved: true,
                      color: Colors.blue,
                      barWidth: 3,
                      isStrokeCapRound: true,
                      dotData: const FlDotData(show: false),
                    ),
                  ],
                ),
              ),
            ),
            const SizedBox(height: 8),
            Row(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                _LegendItem(color: Colors.red, label: 'Temperature (°C)'),
                const SizedBox(width: 16),
                _LegendItem(color: Colors.blue, label: 'Humidity (%)'),
              ],
            ),
          ],
        ),
      ),
    );
  }
}

class _LegendItem extends StatelessWidget {
  final Color color;
  final String label;

  const _LegendItem({required this.color, required this.label});

  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        Container(
          width: 12,
          height: 12,
          color: color,
        ),
        const SizedBox(width: 4),
        Text(label, style: const TextStyle(fontSize: 12)),
      ],
    );
  }
}
