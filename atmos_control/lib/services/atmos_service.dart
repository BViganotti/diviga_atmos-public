import 'dart:convert';
import 'package:http/http.dart' as http;
import '../models/atmos_data.dart';

class AtmosService {
  final String baseUrl;

  AtmosService({required this.baseUrl});

  Future<AtmosData> getAtmosData() async {
     try {
       print('Fetching data from: $baseUrl/api/atmosphere/full');
       final response = await http.get(Uri.parse('$baseUrl/api/atmosphere/full'));
       print('Response status: ${response.statusCode}');
       print('Response body: ${response.body}');
       if (response.statusCode == 200) {
         final jsonData = json.decode(response.body);
         print('Decoded JSON: $jsonData');
         return AtmosData.fromJson(jsonData);
       } else {
         throw Exception('Server returned ${response.statusCode}: ${response.body}');
       }
     } catch (e, stackTrace) {
       print('Error fetching data: $e');
       print('Stack trace: $stackTrace');
       rethrow;
     }
   }

  Future<Map<String, dynamic>> changeRelayStatus(String device) async {
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

    try {
      final response = await http.post(Uri.parse('$baseUrl$endpoint'));
      print('Response status: ${response.statusCode}');
      print('Response body: ${response.body}');

      if (response.statusCode == 200) {
        return json.decode(response.body);
      } else {
        throw Exception('Server returned ${response.statusCode}: ${response.body}');
      }
    } catch (e, stackTrace) {
      print('Error changing relay status: $e');
      print('Stack trace: $stackTrace');
      rethrow;
    }
  }
}
