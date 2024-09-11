import 'package:flutter/material.dart';

class RelayCard extends StatelessWidget {
  final String title;
  final String status;
  final VoidCallback onToggle;
  final IconData icon;

  const RelayCard({
    Key? key,
    required this.title,
    required this.status,
    required this.onToggle,
    required this.icon,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Card(
      elevation: 2,
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
      child: ListTile(
        leading: Icon(icon, color: Theme.of(context).primaryColor),
        title: Text(title),
        trailing: Switch(
          value: status == 'On',
          onChanged: (_) => onToggle(),
          activeColor: Theme.of(context).primaryColor,
        ),
        subtitle: Text(
          'Status: $status',
          style: TextStyle(
            color: status == 'On' ? Colors.green : Colors.red,
          ),
        ),
      ),
    );
  }
}
