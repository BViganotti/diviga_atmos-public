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
    }
  }

  Future<void> toggleRelay(String device) async {
    try {
      await _service.toggleRelay(device);
      await fetchAtmosData();
    } catch (e) {
      print('Error toggling $device: $e');
    }
  }
}
