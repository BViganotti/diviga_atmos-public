import 'package:flutter/foundation.dart';
import '../models/atmos_data.dart';
import '../services/atmos_service.dart';

class AtmosProvider with ChangeNotifier {
  final AtmosService _service;
  AtmosData? _atmosData;
  Map<String, RelayStatus> _relayStatuses = {};

  AtmosProvider(this._service);

  AtmosData? get atmosData => _atmosData;
  Map<String, RelayStatus> get relayStatuses => _relayStatuses;

  Future<void> fetchAllData() async {
    try {
      _atmosData = await _service.getAtmosData();
      _relayStatuses = await _service.getAllStatuses();
      notifyListeners();
    } catch (e) {
      print('Error fetching all data: $e');
    }
  }

  RelayStatus? getRelayStatus(String device) => _relayStatuses[device];

  Future<void> changeRelayStatus(String device) async {
    try {
      final relayStatus = await _service.changeRelayStatus(device);
      _relayStatuses[device] = relayStatus;
      await fetchAllData();
      notifyListeners();
    } catch (e) {
      print('Error changing $device status: $e');
    }
  }

  Future<List<AtmosData>> fetchAtmosHistory(String timeRange) async {
    try {
      return await _service.fetchAtmosHistory(timeRange);
    } catch (e) {
      print('Error fetching atmosphere history: $e');
      rethrow;
    }
  }
}
