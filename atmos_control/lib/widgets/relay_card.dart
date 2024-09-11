import 'package:flutter/material.dart';

class RelayCard extends StatelessWidget {
  final String title;
  final bool isOn;
  final VoidCallback onToggle;

  const RelayCard({
    Key? key,
    required this.title,
    required this.isOn,
    required this.onToggle,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Card(
      child: Padding(
        padding: EdgeInsets.all(16),
        child: Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            Text(
              title,
              style: Theme.of(context).textTheme.titleLarge,
            ),
            Switch(
              value: isOn,
              onChanged: (_) => onToggle(),
            ),
          ],
        ),
      ),
    );
  }
}
