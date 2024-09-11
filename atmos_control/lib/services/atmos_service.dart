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

  Future<void> toggleRelay(String device) async {
    final response = await http.post(Uri.parse('$baseUrl/api/devices/$device/toggle'));
    if (response.statusCode != 200) {
      throw Exception('Failed to toggle $device');
    }
  }
}
