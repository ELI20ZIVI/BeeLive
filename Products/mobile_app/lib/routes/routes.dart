import 'package:auto_route/auto_route.dart';
import 'routes.gr.dart';

@AutoRouterConfig()
class BeeLiveRouter extends $BeeLiveRouter {
  @override
  List<AutoRoute> get routes => [
        AutoRoute(
          page: HomeRoute.page,
          usesPathAsKey: true,
          path: '/',
        ),

        /// routes go here
      ];
}
