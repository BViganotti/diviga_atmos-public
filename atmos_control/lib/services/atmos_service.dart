import 'dart:convert';
import 'package:http/http.dart' as http;
import '../models/atmos_data.dart';

class RelayStatus {
  final bool success;
  final String message;
  final String previousStatus;
  final String newStatus;
  final String lastTurnOn;
  final String lastTurnOff;

  RelayStatus({
    required this.success,
    required this.message,
    required this.previousStatus,
    required this.newStatus,
    required this.lastTurnOn,
    required this.lastTurnOff,
  });

  factory RelayStatus.fromJson(Map<String, dynamic> json) {
    return RelayStatus(
      success: json['success'] ?? false,
      message: json['response'] ?? '',
      previousStatus: json['previous_status'] ?? '',
      newStatus: json['new_status'] ?? '',
      lastTurnOn: json['last_turn_on'] ?? '',
      lastTurnOff: json['last_turn_off'] ?? '',
    );
  }
}

class AtmosService {
  final String baseUrl;

  AtmosService(this.baseUrl);

  Future<AtmosData> getAtmosData() async {
    try {
      print('Fetching data from: $baseUrl/api/atmosphere/full');
      final response =
          await http.get(Uri.parse('$baseUrl/api/atmosphere/full'));
      print('Response status: ${response.statusCode}');
      print('Response body: ${response.body}');
      if (response.statusCode == 200) {
        final jsonData = json.decode(response.body);
        print('Decoded JSON: $jsonData');
        return AtmosData.fromJson(jsonData);
      } else {
        throw Exception(
            'Server returned ${response.statusCode}: ${response.body}');
      }
    } catch (e, stackTrace) {
      print('Error fetching data: $e');
      print('Stack trace: $stackTrace');
      rethrow;
    }
  }

  Future<RelayStatus> changeRelayStatus(String device) async {
    String endpoint;
    switch (device.toLowerCase()) {
      case 'fridge':
        endpoint = '/change_fridge_status';
        break;
      case 'humidifier':
        endpoint = '/change_humidifier_status';
        break;
      case 'dehumidifier':
        endpoint = '/change_dehumidifier_status';
        break;
      case 'ventilator':
        endpoint = '/change_ventilator_status';
        break;
      default:
        throw ArgumentError('Invalid device: $device');
    }

    print('Fetching data from: $baseUrl$endpoint');

    try {
      final response = await http.post(Uri.parse('$baseUrl$endpoint'));
      print('Response status: ${response.statusCode}');
      print('Response body: ${response.body}');

      if (response.statusCode == 200) {
        final jsonData = json.decode(response.body);
        print('Decoded JSON: $jsonData');
        return RelayStatus.fromJson(jsonData);
      } else {
        throw Exception(
            'Server returned ${response.statusCode}: ${response.body}');
      }
    } catch (e, stackTrace) {
      print('Error changing relay status: $e');
      print('Stack trace: $stackTrace');
      rethrow;
    }
  }

  Future<List<AtmosData>> fetchAtmosHistory(String timeRange) async {
    try {
      final response = await http
          .get(Uri.parse('$baseUrl/api/atmosphere/history?range=$timeRange'));
      print('Response status: ${response.statusCode}');
      print('Response body: ${response.body}');

      if (response.statusCode == 200) {
        final List<dynamic> jsonData = json.decode(response.body);
        return jsonData.map((item) => AtmosData.fromJson(item)).toList();
      } else {
        throw Exception(
            'Failed to load atmosphere data: ${response.statusCode}');
      }
    } catch (e, stackTrace) {
      print('Error fetching atmosphere history: $e');
      print('Stack trace: $stackTrace');
      rethrow;
    }
  }
}
