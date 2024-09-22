import 'package:flutter/material.dart';
import '../services/atmos_service.dart';

class RelayCard extends StatelessWidget {
  final String title;
  final RelayStatus? relayStatus;
  final VoidCallback onToggle;
  final IconData icon;

  const RelayCard({
    super.key,
    required this.title,
    required this.relayStatus,
    required this.onToggle,
    required this.icon,
  });

  @override
  Widget build(BuildContext context) {
    final status = relayStatus?.newStatus ?? 'Unknown';
    final lastTurnOn = relayStatus?.lastTurnOn ?? 'N/A';
    final lastTurnOff = relayStatus?.lastTurnOff ?? 'N/A';

    return Card(
      elevation: 2,
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
      child: ExpansionTile(
        leading: Icon(icon, color: Theme.of(context).primaryColor),
        title: Text(title),
        trailing: Switch(
          value: status.toLowerCase() == 'on',
          onChanged: (_) => onToggle(),
          activeColor: Theme.of(context).primaryColor,
        ),
        children: [
          ListTile(
            title: Text('Status: $status'),
            subtitle: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text('Last turned on: $lastTurnOn'),
                Text('Last turned off: $lastTurnOff'),
              ],
            ),
          ),
        ],
      ),
    );
  }
}
