import 'package:flutter/foundation.dart';
import '../models/atmos_data.dart';
import '../services/atmos_service.dart';

class AtmosProvider with ChangeNotifier {
  final AtmosService _service;
  AtmosData? _atmosData;

  AtmosProvider(this._service);

  AtmosData? get atmosData => _atmosData;

  Future<void> fetchAtmosData() async {
    try {
      _atmosData = await _service.getAtmosData();
      notifyListeners();
    } catch (e) {
      print('Error fetching atmos data: $e');
      // Optionally, you can set an error state here
      // _error = e.toString();
      notifyListeners();
    }
  }

  Future<void> changeRelayStatus(String device) async {
    try {
      await _service.changeRelayStatus(device);
      await fetchAtmosData();
    } catch (e) {
      print('Error changing $device status: $e');
    }
  }
}
