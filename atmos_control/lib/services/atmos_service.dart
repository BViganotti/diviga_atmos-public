import 'dart:convert';
import 'package:http/http.dart' as http;
import '../models/atmos_data.dart';

class AtmosService {
  final String baseUrl;

  AtmosService({required this.baseUrl});

  Future<AtmosData> getAtmosData() async {
    final response = await http.get(Uri.parse('$baseUrl/api/atmosphere/full'));
    if (response.statusCode == 200) {
      return AtmosData.fromJson(json.decode(response.body));
    } else {
      throw Exception('Failed to load atmos data');
    }
  }

  Future<void> toggleRelay(String device) async {
    final response = await http.post(Uri.parse('$baseUrl/api/devices/$device/toggle'));
    if (response.statusCode != 200) {
      throw Exception('Failed to toggle $device');
    }
  }
}
